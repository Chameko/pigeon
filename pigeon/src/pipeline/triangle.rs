use std::ops::Deref;
use parrot::{VertexBuffer, IndexBuffer, pipeline::{PipelineCore, PipelineDescription, Set, Pipeline}, vertex::VertexFormat, Plumber, binding::{Binding, BindingType}, Painter, buffers::UniformBuffer, RenderPassExtention, transform::ScreenSpace};
use wgpu::RenderPass;
use super::{VERTEX_INIT_SIZE, INDEX_INIT_SIZE, RenderInformation, Render};
use euclid::Transform3D;

/// A pipeline which doesn't have any texturing capabilities. Instead it has a color for each vertex
/// Useful for drawing primatives
#[derive(Debug)]
pub struct TrianglePipe {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    core: PipelineCore,
}

impl Deref for TrianglePipe {
    type Target = PipelineCore;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl<'a> Plumber<'a> for TrianglePipe {
    type PrepareContext = RenderInformation<TriangleVertex>;
    type Uniforms = [[f32;4];4];

    fn description() -> PipelineDescription<'a> {
        PipelineDescription {
            vertex_layout: &TriangleVertex::VERTEX_LAYOUT,
            pipeline_layout: Some(&[
                Set(&[
                    Binding {
                        binding: BindingType::UniformBuffer,
                        stage: wgpu::ShaderStages::VERTEX,
                    }
                ], Some("Triangle transform bind group"))
            ]),
            shader: parrot::shader::ShaderFile::Wgsl(include_str!("./shaders/triangle.wgsl")),
            name: Some("Triangle pipeline"),
        }
    }

    fn setup(pipe: Pipeline, paint: &Painter) -> Self {
        // Allocating a bunch of capacity for the buffers to prevent resizing them 1000 times
        let blank_vertex: Vec<TriangleVertex> = Vec::with_capacity(VERTEX_INIT_SIZE as usize);
        let blank_index: Vec<u16> = Vec::with_capacity(INDEX_INIT_SIZE as usize);
        let blank_transform: Transform3D<f32, ScreenSpace, ScreenSpace> = Transform3D::identity();

        let vertex_buffer = paint.vertex_buffer(blank_vertex.as_slice(), Some("Triangle vertex buffer"));
        let index_buffer = paint.index_buffer(blank_index.as_slice(), Some("Triangle index buffer"));
        let transform_buffer = paint.uniform_buffer(&[blank_transform.to_arrays()], Some("Triangle transform buffer"));
        let bind_group = paint.binding_group(&pipe.layout.b_layouts[0], &[&transform_buffer], Some("Triangle transform binding group"));

        Self {
            vertex_buffer,
            index_buffer,
            core: PipelineCore {
                pipeline: pipe,
                bindings: vec![bind_group],
                uniforms: vec![transform_buffer]
            }
        }
    }

    fn prepare(&'a mut self, prep: Self::PrepareContext, paint: &mut Painter) -> Vec<(&'a mut UniformBuffer, Vec<Self::Uniforms>)> {
        let mut vertices: Vec<TriangleVertex> = vec![];
        let mut indices: Vec<u16> = vec![];

        // Combine into a big ol array.
        for mut tri in prep.0 {
            let start = vertices.len();
            vertices.append(&mut tri.vertices);
            indices.append(&mut tri.indicies.iter().map(|ind| ind + start as u16).collect());
        }

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

impl Render for TrianglePipe {
    type Vertex = TriangleVertex;

    fn render<'a>(&'a mut self, _paint: &mut Painter, pass: &mut RenderPass<'a>) {
        // Set pipeline
        pass.set_parrot_pipeline(self);

        // Set buffers
        pass.set_parrot_vertex_buffer(&self.vertex_buffer);
        pass.set_parrot_index_buffer(&self.index_buffer);
        

        // Draw
        pass.draw_parrot_indexed(0..self.index_buffer.size, 0..1);
    }
}

/// The vertex for triangles
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleVertex {
    /// Position of the vertex in worldspace
    pub pos: [f32; 3],
    /// The color of the vertex
    pub color: [f32; 4],
}

impl Default for TriangleVertex {
    fn default() -> Self {
        Self {
            pos: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0]
        }
    }
}

impl TriangleVertex {
    pub fn for_primative(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: [x, y, z],
            ..Default::default()
        }
    }

    pub fn new(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            pos: [x, y, z],
            color: [r, g, b, a]
        }
    }

    pub fn new_from_tuple(pos: (f32, f32, f32), col: (f32, f32, f32, f32)) -> Self {
        Self {
            pos: [pos.0, pos.1, pos.2],
            color: [col.0, col.1, col.2, col.3],
        }
    }

    pub const VERTEX_LAYOUT: [VertexFormat; 2] = [
        VertexFormat::Floatx3,
        VertexFormat::Floatx4,
    ];
}