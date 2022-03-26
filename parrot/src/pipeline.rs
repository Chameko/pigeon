use std::ops::Deref;

use crate::{
    binding::{
        BindingGroupLayout,
        BindingGroup,
        Binding,
    },
    vertex::{VertexLayout, VertexFormat},
    shader::ShaderFile,
    device::Device,
    buffers::uniform::UniformBuffer,
};

#[derive(Debug)]
/// Represents a pipeline
pub struct Pipeline {
    /// Wrapper around [`wgpu::RenderPipeline`]
    pub wgpu: wgpu::RenderPipeline,
    /// Layout of the pipeline
    pub layout: PipelineLayout,
    /// Layout of the verticies in the pipeline
    pub vertex_layout: VertexLayout,
}

#[derive(Debug)]
pub struct PipelineLayout {
    pub b_layouts: Vec<BindingGroupLayout>,
}

/// A trait for managing pipelines and their functionality
pub trait Plumber<'a>: Deref<Target = PipelineCore> {
    /// The vertex type you're using
    type Vertex: bytemuck::Pod + Copy + 'static;

    /// A type that can be used with prepare
    type PrepareContext;

    /// Your uniforms
    type Uniforms: bytemuck::Pod + Copy + 'static;

    /// Returns a [`PipelineDescription`]. This describes the layout of vertecies, sets of bindings and your shader file
    fn description() -> PipelineDescription<'a>;

    /// This is what will create your pipeline.
    fn setup(pipe: Pipeline, device: &Device) -> Self;

    /// Prepare the uniform buffers with the supplied PrepareContext.
    fn prepare(&'a mut self, context: Self::PrepareContext) -> Vec<(&'a mut UniformBuffer, Vec<Self::Uniforms>)>;

    /// Returns the pipeline's name, used for debugging purposes
    fn name() -> String;
}

#[derive(Debug)]
/// The core components of a pipeline. [`Plumber`] derefs to this during a render pass.
pub struct PipelineCore {
    pub pipeline: Pipeline,
    pub bindings: Option<BindingGroup>,
    pub uniforms: Option<UniformBuffer>,
}

#[derive(Debug)]
/// A Set of bindings
pub struct Set<'a>(pub &'a[Binding], pub Option<&'a str>);

#[derive(Debug)]
/// Description used to create pipelines
pub struct PipelineDescription<'a> {
    /// Vertex layout of the pipeline
    pub vertex_layout: &'a [VertexFormat],
    /// Bindings used to create a pipeline layout
    pub pipeline_layout: Option<&'a [Set<'a>]>,
    /// Shader file
    pub shader: ShaderFile
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blending {
    src_factor: BlendFactor,
    dst_factor: BlendFactor,
    operation: BlendOp,
}

impl Blending {
    pub fn new(src_factor: BlendFactor, dst_factor: BlendFactor, operation: BlendOp) -> Self {
        Blending {
            src_factor,
            dst_factor,
            operation,
        }
    }

    pub fn constant() -> Self {
        Blending {
            src_factor: BlendFactor::One,
            dst_factor: BlendFactor::Zero,
            operation: BlendOp::Add,
        }
    }

    pub fn as_wgpu(&self) -> (wgpu::BlendFactor, wgpu::BlendFactor, wgpu::BlendOperation) {
        (
            self.src_factor.as_wgpu(),
            self.dst_factor.as_wgpu(),
            self.operation.as_wgpu()
        )
    }
}

impl Default for Blending {
    fn default() -> Self {
        Blending {
            src_factor: BlendFactor::SrcAlpha,
            dst_factor: BlendFactor::OneMinusSrcAlpha,
            operation: BlendOp::Add,
        }
    }
}

/// Wrapper around [`wgpu::BlendFactor`]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendFactor {
    One,
    Zero,
    SrcAlpha,
    OneMinusSrcAlpha,
}

impl BlendFactor {
    fn as_wgpu(&self) -> wgpu::BlendFactor {
        match self {
            BlendFactor::SrcAlpha => wgpu::BlendFactor::SrcAlpha,
            BlendFactor::OneMinusSrcAlpha => wgpu::BlendFactor::OneMinusSrcAlpha,
            BlendFactor::One => wgpu::BlendFactor::One,
            BlendFactor::Zero => wgpu::BlendFactor::Zero,
        }
    }
}

impl From<BlendFactor> for wgpu::BlendFactor {
    fn from(blend_f: BlendFactor) -> Self {
        blend_f.as_wgpu()
    }
}

/// Wrapper around [`wgpu::BlendOperation`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendOp {
    Add,
}

impl BlendOp {
    fn as_wgpu(&self) -> wgpu::BlendOperation {
        match self {
            BlendOp::Add => wgpu::BlendOperation::Add,
        }
    }
}

impl From<BlendOp> for wgpu::BlendOperation {
    fn from(b_op: BlendOp) -> Self {
        b_op.as_wgpu()
    }
}