use std::ops::Deref;
use std::collections::HashMap;

use pigeon_parrot as parrot;
use crate::error::EguiBackendError;
use parrot::{
    pipeline::{Plumber,
        PipelineDescription,
        Set,
        PipelineCore,
        Pipeline,
    },
    buffers::{
        UniformBuffer,
        VertexBuffer,
        IndexBuffer
    },
    vertex::VertexFormat,
    shader::ShaderFile,
    binding::{
        Binding,
        BindingType
    },
    device::Device,
    transform::ScreenSpace,
    texture::Texture,
    Painter
};
use egui::{
    epaint::Vertex,
    epaint::ClippedMesh,
    TextureId,
    ImageData,
    Color32,
};
use euclid::{
    Size2D,
    Point2D,
    Rect,
};

type Result<T> = std::result::Result<T, EguiBackendError>;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    screen_size: [f32; 2]
}

/// Pipeline for egui
/// 
/// # Usage
/// This isn't intended to be used directly, but instead be controlled by [Egui]
pub struct EguiPipe {
    /// The vertex buffer, managed by egui
    vertex_buffer: Vec<VertexBuffer>,
    /// The index buffer
    index_buffer: Vec<IndexBuffer>,
    /// Textures managed by egui
    textures: HashMap<egui::TextureId, Texture>,
    /// Uniform buffer
    uniform_buffer: TransformUniform,
}

impl EguiPipe {
    /// Update and set egui textures. Call before executing a render pass
    pub fn set_textures(&mut self, textures: &egui::TexturesDelta, paint: &Painter) -> Result<()> {
        // Set or update any textures needed
        for (tex_id, tex_data) in textures.set.iter() {
            // Get the texture
            if let Some(tex) = self.textures.get(tex_id) {
                // Update texture
                if let Some(raw_pos) = tex_data.pos {
                    // Update particular region
                    match &tex_data.image {
                        ImageData::Alpha(a) => {
                            let size: Size2D<usize, ScreenSpace> = Size2D::new(a.width(), a.height());
                            let pos: Point2D<usize, ScreenSpace> = raw_pos.into();
                            let rect = Rect::new(pos, size).to_u32();
                            let pixels = a.srgba_pixels(1.0).collect::<Vec<_>>();
                            Texture::transfer(tex, pixels.as_slice(), rect, &paint.device);
                        }
                        ImageData::Color(c) => {
                            let size: Size2D<usize, ScreenSpace> = Size2D::new(c.width(), c.height());
                            let pos: Point2D<usize, ScreenSpace> = raw_pos.into();
                            let rect = Rect::new(pos, size).to_u32();
                            Texture::transfer(tex, c.pixels.as_slice(), rect, &paint.device);
                        }
                    }
                } else {
                    // Update entire texture
                    match &tex_data.image {
                        ImageData::Alpha(a) => {
                            let pixels = a.srgba_pixels(1.0).collect::<Vec<Color32>>();
                            Texture::fill(tex, pixels.as_slice(), &paint.device);
                        }
                        ImageData::Color(c) => {
                            Texture::fill(tex, c.pixels.as_slice(), &paint.device);
                        }
                    }
                }
            } else {
                if !tex_data.pos.is_some() {
                    // Create texture
                    match &tex_data.image {
                        ImageData::Alpha(a) => {
                            let size: Size2D<u32, ScreenSpace> = Size2D::new(a.width(), a.height()).to_u32();
                            let tex = paint.texture(size, wgpu::TextureFormat::Rgba8UnormSrgb, wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING, None);
                            let pixels = a.srgba_pixels(1.0).collect::<Vec<Color32>>();
                            Texture::fill(&tex, pixels.as_slice(), &paint.device);
                            self.textures.insert(tex_id.clone(), tex);
                        }
                        ImageData::Color(c) => {
                            let size:  Size2D<u32, ScreenSpace> = Size2D::new(c.width(), c.height()).to_u32();
                            let tex = paint.texture(size, wgpu::TextureFormat::Rgba8UnormSrgb, wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING, None);
                            Texture::fill(&tex, c.pixels.as_slice(), &paint.device);
                            self.textures.insert(*tex_id, tex);
                        }
                    }
                } else {
                    let message = format!("Couldn't find texture id: {:?}", tex_id);
                    log::error!("{}", &message);
                    return Err(EguiBackendError::UnknownTextureId(message))
                }
            }
        }
        Ok(())
    }

    /// Free textures. Call after a render pass
    pub fn free_texture(&mut self, textures: &egui::TexturesDelta) -> Result<()> {
        for tex_id in &textures.free {
            if let Some(tex) = self.textures.remove(tex_id) {
                tex.wgpu.destroy()
            } else {
                let message = format!("Attempted to remove unknown texture id: {:?}", tex_id);
                log::error!("{}", message);
                return Err(EguiBackendError::UnknownTextureId(message))
            }
        }
        Ok(())
    }

    /// Update the various buffers
    fn update_buffers(&mut self, full_meshes: Vec<egui::Mesh>, paint: &mut Painter, logical_size: Size2D<u32, ScreenSpace>) {
        // Update uniform buffewr 
        paint.update_pipeline(self, logical_size);

        // Convert meshes to 16 bit indicies, used for compatibility
        let mut meshes: Vec<egui::epaint::Mesh16> = vec![];
        for m in full_meshes {
            meshes.append(&mut m.split_to_u16());
        }
        
        // Update vertex and index buffers
        for (i, mesh) in meshes.iter().enumerate() {
            // Create new buffers as needed
            if i < self.index_buffer.len() {
                // Replace the index buffer if the new one has more data
                if mesh.indices.len() > self.index_buffer[i].elements as usize {
                    self.index_buffer[i] = paint.index_buffer(&mesh.indices);
                } else {
                    paint.update_buffer(mesh.indices.as_slice(), &self.index_buffer[i])
                }
            } else {
                self.index_buffer.push(paint.index_buffer(mesh.indices.as_slice()));
            }

            if i < self.vertex_buffer.len() {
                if bytemuck::cast_slice::<_, u8>(&mesh.vertices).len() > self.vertex_buffer[i].size as usize {
                    self.vertex_buffer[i] = paint.vertex_buffer(&mesh.vertices, Some(&format!("Egui vertex buffer {}", i)));
                } else {
                    paint.update_buffer(mesh.vertices.as_slice(), &self.vertex_buffer[i]);
                }
            } else {
                self.vertex_buffer.push(paint.vertex_buffer(mesh.vertices.as_slice(), Some(&format!("Egui vertex buffer {}", i))));
            }
        }
    }

}

impl Deref for EguiPipe {
    type Target = PipelineCore;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<'a> Plumber<'a> for EguiPipe {
    type Vertex = Vertex;
    type Uniforms = TransformUniform;
    type PrepareContext = Size2D<u32, ScreenSpace>;

    fn description() -> PipelineDescription<'a> {
        PipelineDescription {
            // Position, UV and colour data
            vertex_layout: &[VertexFormat::Floatx2, VertexFormat::Floatx2, VertexFormat::Uint32],
            pipeline_layout: Some(&[
                // Texture bindings
                Set(&[
                    Binding {
                        binding: BindingType::Texture,
                        stage: wgpu::ShaderStages::FRAGMENT,
                    },
                    Binding {
                        binding: BindingType::Sampler,
                        stage: wgpu::ShaderStages::FRAGMENT,
                    },
                ], Some("Tex bind group")),
                // Uniform bindings
                Set(&[
                    Binding {
                        binding: BindingType::UniformBuffer,
                        stage: wgpu::ShaderStages::VERTEX,
                    }
                ], Some("Transform bind group"))
            ]),
            shader: ShaderFile::Wgsl(include_str!("./egui.wgsl"))
        }
    }

    fn prepare(&'a self, context: Self::PrepareContext) -> Option<(&'a UniformBuffer, Vec<Self::Uniforms>)> {
        
        todo!()
    }

    fn setup(pipe: Pipeline, device: &Device) -> Self {
        todo!()
    }

    fn name() -> String {
        "Egui".to_string()
    }
}