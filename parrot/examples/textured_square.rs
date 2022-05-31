extern crate pigeon_parrot as parrot;

use std::{ops::Deref};
use parrot::{
    pipeline::{Plumber,
        PipelineDescription,
        PipelineCore,
        Pipeline, Blending, BlendFactor, BlendOp, Set
    },
    buffers::{UniformBuffer, VertexBuffer, IndexBuffer},
    vertex::VertexFormat,
    shader::ShaderFile,
    painter::Painter,
    painter::PassOp, RenderPassExtention, binding::{Binding, BindingType }, texture::Texture
};
use wgpu::ShaderStages;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use euclid::Size2D;

/// =======================================================================
///  I recommend reading [learn wgpu](https://sotrh.github.io/learn-wgpu/)
///  before using parrot and reading this example
/// 
/// This example is intended to show the use of textures in parrot
/// =======================================================================

/// The struct that will represent our verticies. As we are only drawing 2D shapes, we have a position composed of 2 floats
/// The vertices must implement [`bytemuck::Pod`], [`bytemuck::Zeroable`] and [`Copy`]
/// They also must be #[repr(C)]
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl Vertex {
    // Create a new vertex
    pub fn new(x: f32, y: f32, tx: f32, ty: f32) -> Self {
        Self {
            position: [x,y],
            tex_coords: [tx,ty],
        }
    }
}

/// This is our pipeline, it should contain everything we need as so we can deref to [`PipelineCore`] when we render.
pub struct TrianglePipe {
    /// This is the pipeline core that we will return. Normally you would instead store your [`parrot::binding::BindingGroup`] and [`parrot::buffers::UniformBuffer`] and create it from those, but I won't bother here.
    pipeline: PipelineCore,
    /// The vertex buffer for our pipeline
    vertices: VertexBuffer,
    // The index buffer for our pipeline
    index: IndexBuffer
}

/// This is what allows our rendering functions to render with our pipeline
impl Deref for TrianglePipe {
    type Target = PipelineCore;

    /// This function returns our [`PipelineCore`]. Normally we would create this, but as this is a simple triangle pipeline, I won't bother
    fn deref(&self) -> &Self::Target {
        &self.pipeline
    }
}

impl<'a> Plumber<'a> for TrianglePipe {
    /// Typically contains the information required to update our uniform buffer. We don't have one so we use ()
    type PrepareContext = ();
    /// Once again we have no uniforms, so we use ()
    type Uniforms = ();

    /// This is the function that will be used to create our pipeline
    fn setup(pipe: Pipeline, painter: &Painter) -> Self {
        // The four default verticies that will make up our quad. If you want, this can be blank.
        let vertices = [Vertex::new(-0.5, 0.5, 0.0, 0.0), Vertex::new(0.5, 0.5, 1.0, 0.0), Vertex::new(0.5, -0.5, 1.0, 1.0), Vertex::new(-0.5, -0.5, 0.0, 1.0)];
        // Create a vertex buffer, ours contains three verticies
        let vertex = painter.vertex_buffer::<Vertex>(&vertices, Some("Triangle vertex buffer"));
        // Create the index buffer, ours needs two triangles to make our quad
        let index = painter.index_buffer(&[0, 1, 2,
                                                                2, 3, 0], Some("Triangle index buffer"));

        // Load our image
        let img_bytes = include_bytes!("./logo.png");
        let img = image::load_from_memory(img_bytes).unwrap();
        // Convert to our colour format
        let img_rgb = img.to_rgba8().to_vec();
        println!("Length: {}", img_rgb.len());
        let img_rgb = parrot::color::Rgba8::align(img_rgb.as_slice());
        println!("Alligned length: {}", img_rgb.len());

        use image::GenericImageView;
        let dimensions = img.dimensions();

        // Create an empty texture
        let texture = painter.texture(Size2D::from(dimensions), wgpu::TextureFormat::Rgba8UnormSrgb, wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, Some("logo"), false);
        // Fill the texture with the image bytes
        Texture::fill(&texture, img_rgb, &painter.device);
        // Create a sampler for our texture
        let sampler = painter.sampler(wgpu::FilterMode::Nearest, wgpu::FilterMode::Linear, Some("Image sampler"));

        // Create relevant bindings
        let texture_layout = &pipe.layout.b_layouts[0];

        let texture_bind = painter.binding_group(texture_layout, &[&texture, &sampler], Some("Texture bind group"));

        let pipeline = PipelineCore {
            // The actuall pipeline
            pipeline: pipe,
            // Our bindings (we have one texture)
            bindings: vec![texture_bind],
            // Our uniforms (we have none)
            uniforms: vec![]
        };

        Self {
            pipeline,
            vertices: vertex,
            index,
        }
    }

    // This function is used to update our uniform buffer. As we don't have one, we return a blank vector
    fn prepare(&'a mut self, _context: Self::PrepareContext, _: &mut Painter) -> Vec<(&'a mut UniformBuffer, Vec<Self::Uniforms>)> {
        vec![]
    }

    // This function is used to describe the layout of our pipeline
    fn description() -> PipelineDescription<'a> {
        PipelineDescription {
            vertex_layout: &[VertexFormat::Floatx2, VertexFormat::Floatx2], // Layout of 2 floats for position, 2 floats for texture coords
            pipeline_layout: Some(&[
                // Add a set of bindings
                Set(&[
                    // Our texture binding
                    Binding {
                        binding: BindingType::Texture { multisampled: false },
                        stage: ShaderStages::FRAGMENT, // We'll use this in the vertex stage
                    },
                    Binding {
                        binding: BindingType::Sampler, // A sampler for our texture
                        stage: ShaderStages::FRAGMENT, // We'll use this in the fragment stage
                    }
                    ], Some("Triangle texture bind group") // A name, not neccessary but usefull for debugging
                )
            ]),
            shader: ShaderFile::Wgsl(include_str!("./shaders/textured_square.wgsl")), // Takes in triangle shader
            name: Some("Triangle pipeline") // Name of pipeline
        }
    }
}

fn main() {
    // Initialise the logging output at info level only from parrot
    env_logger::builder().filter_module("pigeon_parrot", log::LevelFilter::Info).init();

    // Create an event loop
    let event_loop = winit::event_loop::EventLoop::new();
    // Create a window to draw to
    let window = winit::window::WindowBuilder::new().with_title("Triangle :D").build(&event_loop).unwrap();

    // Create a wgpu instance
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };

    // Create the painter
    let mut painter = pollster::block_on(parrot::Painter::for_surface(surface, &instance, 1)).unwrap();

    // Get the preferred texture format for the s
    let pref_format = painter.preferred_format();

    // Get the size of the window
    let winsize = window.inner_size();

    // Configure the surface
    painter.configure(euclid::Size2D::new(winsize.width, winsize.height), wgpu::PresentMode::Fifo, pref_format);

    // Create our pipeline with no depth buffer :D
    let blending = Blending::new(BlendFactor::One, BlendFactor::Zero, BlendOp::Add);
    let pipeline = painter.pipeline_no_depth::<TrianglePipe>(blending, pref_format, Some("Triangle shader"));

    // Initiate the event loop
    event_loop.run(move |event, _, control_flow| {
        // Only update the event loop if input is recieved
        *control_flow = ControlFlow::Wait;

        match event {
            // Window event
            Event::WindowEvent { event: win_event, .. } => {
                match win_event {
                    // Close if a close request is detected
                    WindowEvent::CloseRequested => {
                        println!("The close button was pressed; stopping");
                        *control_flow = ControlFlow::Exit
                    },
                    // Update the surface if resized
                    WindowEvent::Resized(size) => {
                        let size = euclid::Size2D::new(size.width, size.height);
                        painter.configure(size, wgpu::PresentMode::Fifo, pref_format)
                    }
                    _ => ()
                }
            },
            Event::RedrawRequested(_) => {
                // Time to draw our shape :D

                // Create a frame. This represents our, well, frame
                let mut frame = painter.frame();

                // Grab the current surface, we grab the one with no depth buffer attached
                let current_surface = painter.current_frame_no_depth().unwrap();

                {
                    // Initiate a render pass
                    let mut pass = frame.pass(PassOp::Clear(parrot::color::Rgba::new(0.1, 0.2, 0.3, 1.0)), &current_surface, None);

                    // Set our pipeline
                    pass.set_parrot_pipeline(&pipeline);

                    // Set our vertex buffer
                    pass.set_parrot_vertex_buffer(&pipeline.vertices);

                    // Set our index buffer
                    pass.set_parrot_index_buffer(&pipeline.index);
                    
                    // Set our bind groups
                    pass.set_binding(&pipeline.bindings[0], &[]);

                    // Perform the render pass on the entire vertex buffer
                    pass.draw_parrot_indexed(0..pipeline.index.size, 0..1);
                }

                // Present our frame
                painter.present(frame);
            }
            _ => ()
        }
    });
}