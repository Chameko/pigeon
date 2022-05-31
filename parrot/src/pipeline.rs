use std::ops::Deref;

use crate::{
    binding::{
        BindingGroupLayout,
        BindingGroup,
        Binding,
    },
    vertex::{VertexLayout, VertexFormat},
    shader::ShaderFile,
    buffers::uniform::UniformBuffer, Painter,
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

/// A trait for creating and managing a pipeline.
/// 
/// This trait is used to effectivly used to create your own pipeline while allowing parrot to perform some of the work.
pub trait Plumber<'a>: Deref<Target = PipelineCore> {
    /// A type that containts the neccissary data for updating the uniform buffer
    type PrepareContext;

    /// Your uniforms
    type Uniforms: bytemuck::Pod + Copy + 'static;

    /// Returns a [`PipelineDescription`]. This describes the layout of vertecies, sets of bindings and your shader file.
    fn description() -> PipelineDescription<'a>;

    /// Used to create your pipeline. Supplies the wgpu pipeline and device.
    fn setup(pipe: Pipeline, painter: &Painter) -> Self;

    /// Create the uniforms neccissary for an update with the supplied [`PrepareContext`].
    fn prepare(&'a mut self, context: Self::PrepareContext, paint: &mut Painter) -> Vec<(&'a mut UniformBuffer, Vec<Self::Uniforms>)>;
}

#[derive(Debug)]
/// The core components of a pipeline. These are used by wgpu when performing a render pass, hence your pipeline must have some method of supplying the information.
pub struct PipelineCore {
    /// The actual pipeline
    pub pipeline: Pipeline,
    /// The bindings to be used in the render pass
    pub bindings: Vec<BindingGroup>,
    /// The uniforms to be used in the render pass
    pub uniforms: Vec<UniformBuffer>,
}

#[derive(Debug)]
/// A Set of bindings
pub struct Set<'a>(pub &'a[Binding], pub Option<&'a str>);

#[derive(Debug)]
/// A description of how a pipeline is laid out. This is used by parrot to create your pipeline.
pub struct PipelineDescription<'a> {
    /// Vertex layout of the pipeline
    pub vertex_layout: &'a [VertexFormat],
    /// Bindings used to create a pipeline layout
    pub pipeline_layout: Option<&'a [Set<'a>]>,
    /// Shader file
    pub shader: ShaderFile,
    /// Name of the pipeline
    pub name: Option<&'a str>
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