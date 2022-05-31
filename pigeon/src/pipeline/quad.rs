use parrot::{
    Plumber,
    buffers::*,
    pipeline::{PipelineCore, Pipeline, PipelineDescription, Set}, binding::{Binding, BindingType},
    vertex::VertexFormat, Painter, painter::RenderPassExtention, transform::{ScreenSpace},
};
use pigeon_parrot::binding::BindingGroup;
use wgpu::RenderPass;
use std::{ops::{Deref, Range}, collections::HashMap};
use euclid::Transform3D;
use crate::graphics::Texture;
use super::{VERTEX_INIT_SIZE, INDEX_INIT_SIZE, RenderInformation, Render};

/// Helps [QuadPipe] know which texture to set depending on how many indicies deep it is in the buffer
#[derive(Debug)]
pub struct Group {
    range: Range<u32>,
    tex_id: usize,
}

/// Pipeline for drawing textured quads. Designed to work with [`crate::graphics::sprite::Sprite`]
#[derive(Debug)]
pub struct QuadPipe {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
    pub groups: Vec<Group>,
    pub texture_binds: HashMap<usize, BindingGroup>,
    /// Pipeline core to deref to
    core: PipelineCore,
}

impl Deref for QuadPipe {
    type Target = PipelineCore;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl<'a> Plumber<'a> for QuadPipe {
    type PrepareContext = RenderInformation<QuadVertex>;
    type Uniforms = [[f32;4];4];

    fn description() -> PipelineDescription<'a> {
        PipelineDescription {
            vertex_layout: &QuadVertex::VERTEX_LAYOUT,
            pipeline_layout: Some(&[
                Set(&[
                    Binding {
                        binding: BindingType::Texture { multisampled: false },
                        stage: wgpu::ShaderStages::FRAGMENT,
                    },
                    Binding {
                        binding: BindingType::Sampler,
                        stage: wgpu::ShaderStages::FRAGMENT,
                    }
                ], Some("Quad texture bind group")),
                Set(&[
                    Binding {
                        binding: BindingType::UniformBuffer,
                        stage: wgpu::ShaderStages::VERTEX,
                    }
                ], Some("Quad transform bind group"))
            ]),
            shader: parrot::shader::ShaderFile::Wgsl(include_str!("./shaders/quad.wgsl")),
            name: Some("Quad pipeline")
        }
    }

    fn setup(pipe: Pipeline, paint: &Painter) -> Self {
        // Allocating a bunch of capacity for the buffers to prevent resizing them 1000 times
        let blank_vertex: Vec<QuadVertex> = Vec::with_capacity(VERTEX_INIT_SIZE as usize);
        let blank_index: Vec<u16> = Vec::with_capacity(INDEX_INIT_SIZE as usize);
        let blank_transform: Transform3D<f32, ScreenSpace, ScreenSpace> = Transform3D::identity();

        let vertex_buffer = paint.vertex_buffer(blank_vertex.as_slice(), Some("Quad vertex buffer"));
        let index_buffer = paint.index_buffer(blank_index.as_slice(), Some("Quad index buffer"));
        let transform_buffer = paint.uniform_buffer(&[blank_transform.to_arrays()], Some("Quad transform buffer"));
        let bind_group = paint.binding_group(&pipe.layout.b_layouts[1], &[&transform_buffer], Some("Quad transform binding group"));

        Self {
            vertex_buffer,
            index_buffer,
            groups: vec![],
            texture_binds: HashMap::new(),
            core: PipelineCore {
                pipeline: pipe,
                bindings: vec![bind_group],
                uniforms: vec![transform_buffer]
            }
        }
    }

    fn prepare(&'a mut self, prep: Self::PrepareContext, paint: &mut Painter) -> Vec<(&'a mut UniformBuffer, Vec<Self::Uniforms>)> {
        let mut vertices: Vec<QuadVertex> = vec![];
        let mut indices: Vec<u16> = vec![];
        let mut groups: Vec<Group> = vec![];

        // Combine into a big ol array.
        for mut quad in prep.0 {
            let start = vertices.len();
            vertices.append(&mut quad.vertices);
            let start2 = indices.len() as u32;
            indices.append(&mut quad.indicies.iter().map(|ind| ind + start as u16).collect());
            if let Some(tex) = quad.texture {
                // Check if we have already bound the texture
                if !self.texture_binds.contains_key(&tex.id) {
                    // Add texture to the map
                    self.add_texture(&paint, &tex);
                }
                groups.push(Group{range: start2..indices.len() as u32, tex_id: tex.id});
            } else {
                panic!("Textured shape has no texture.")
            }
        }

        self.groups = groups;

        // Update the vertex and index buffers
        if let Some(v) = paint.update_vertex_buffer(&vertices, &mut self.vertex_buffer) {
            self.vertex_buffer = v;
        }
        if let Some(i) = paint.update_index_buffer(indices, &mut self.index_buffer) {
            self.index_buffer = i;
        }

        // Return info for parrot to update our uniform buffers
        vec![(&mut self.core.uniforms[0], vec![prep.1.to_arrays()])]
    }
}

impl Render for QuadPipe {
    type Vertex = QuadVertex;

    fn render<'a>(&'a mut self, _paint: &mut Painter, pass: &mut RenderPass<'a>) {
        // Set pipeline
        pass.set_parrot_pipeline(self);

        // Set buffers
        pass.set_parrot_vertex_buffer(&self.vertex_buffer);
        pass.set_parrot_index_buffer(&self.index_buffer);


        // Draw textured shapes
        if let Some(group) = self.groups.first() {
            // Set the first binding
            pass.set_binding(self.texture_binds.get(&group.tex_id).expect("Cannot find texture in textures map"), &[]);
            let mut prev_tex = group.tex_id;
            for g in &self.groups {
                if prev_tex != g.tex_id {
                    pass.set_binding(self.texture_binds.get(&g.tex_id).expect("Cannot find texture in textures map"), &[]);
                    prev_tex = g.tex_id;
                }
                pass.draw_parrot_indexed(g.range.clone(), 0..1);
            }
        }
    }
}

impl QuadPipe {
    pub fn add_texture(&mut self, paint: &Painter, tex: &Texture) {
        let bind_group = paint.binding_group(
            &self.core.pipeline.layout.b_layouts[0],
            &[&tex.texture, &*tex.sampler],
            Some(&format!("{} binding group", tex.name)));
        self.texture_binds.insert(tex.id, bind_group);
    }
}

/// The vertex for quads
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]

pub struct QuadVertex {
    /// Position of the vertex in worldspace
    pub pos: [f32; 3],
    /// The u-v coordinates of the vertex on the texture
    pub tex_coords: [f32; 2],
}

impl Default for QuadVertex {
    fn default() -> Self {
        Self {
            pos: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0]
        }
    }
}

impl QuadVertex {
    pub fn for_primative(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: [x, y, z],
            ..Default::default()
        }
    }

    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self {
        Self {
            pos: [x, y, z],
            tex_coords: [u, v]
        }
    }

    pub fn new_from_tuple(pos: (f32, f32, f32), tex: (f32, f32)) -> Self {
        Self {
            pos: [pos.0, pos.1, pos.2],
            tex_coords: [tex.0, tex.1],
        }
    }

    pub const VERTEX_LAYOUT: [VertexFormat; 2] = [
        VertexFormat::Floatx3,
        VertexFormat::Floatx2,
    ];
}