use std::ops::Deref;

use pigeon_parrot as parrot;
use parrot::{
    pipeline::{Plumber,
        PipelineDescription,
        PipelineCore,
        Pipeline,
    },
    buffers::UniformBuffer,
    vertex::VertexFormat,
    shader::ShaderFile,
    device::Device,
};

pub struct Triangle {
    pipeline: PipelineCore,
}

impl Deref for Triangle {
    type Target = PipelineCore;

    fn deref(&self) -> &Self::Target {
        &self.pipeline
    }
}

impl<'a> Plumber<'a> for Triangle {
    type Vertex = crate::vertex::Vertex;
    type PrepareContext = i32;
    type Uniforms = i32;

    fn setup(pipe: Pipeline, device: &Device) -> Self {
        let pipeline = PipelineCore {
            pipeline: pipe,
            bindings: None,
            uniforms: None
        };

        Self {
            pipeline
        }
    }

    fn prepare(&'a mut self, context: Self::PrepareContext) -> Vec<(&'a mut UniformBuffer, Vec<Self::Uniforms>)> {
        vec![]
    }

    fn description() -> PipelineDescription<'a> {
        PipelineDescription {
            vertex_layout: &[VertexFormat::Floatx2], // Layout of 2 floats
            pipeline_layout: None, // Has no bindings, for now
            shader: ShaderFile::Wgsl(include_str!("../shader/triangle.wgsl")) // Takes in triangle shader
        }
    }

    fn name() -> String {
        "triangle".to_string()
    }
}