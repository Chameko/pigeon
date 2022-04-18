extern crate pigeon_parrot as parrot;

use std::{ops::Deref};
use parrot::{
    pipeline::{Plumber,
        PipelineDescription,
        PipelineCore,
        Pipeline, Blending,
    },
    buffers::{UniformBuffer, VertexBuffer, IndexBuffer},
    vertex::VertexFormat,
    shader::ShaderFile,
    painter::PassOp, RenderPassExtention, Painter,
};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode};
use winit::event_loop::ControlFlow;

/// =======================================================================
///  I recommend reading [learn wgpu](https://sotrh.github.io/learn-wgpu/)
///  before using parrot and reading this example
/// 
/// This example is intended to show how to update buffers using parrot
/// =======================================================================

/// The struct that will represent our verticies. As we are only drawing 2D shapes, we have a position composed of 2 floats
/// The vertices must implement [`bytemuck::Pod`], [`bytemuck::Zeroable`] and [`Copy`]
/// They also must be #[repr(C)]
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    // Create a new vertex
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: [x,y],
        }
    }
}

/// This is our pipeline, it should contain everything we need as so we can deref to [`PipelineCore`] when we render.
pub struct TrianglePipe {
    /// This is the pipeline core that we will return. Normally you would instead store your [`parrot::binding::BindingGroup`] and [`parrot::buffers::UniformBuffer`] and create it from those, but as this is a simple pipeline with no textures or uniforms, I won't bother
    pipeline: PipelineCore,
    /// The vertex buffer for our pipeline
    vertices: VertexBuffer,
    /// The index buffer for our pipeline
    indicies: IndexBuffer
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
        let pipeline = PipelineCore {
            // The actuall pipeline
            pipeline: pipe,
            // Our bindings (we have none)
            bindings: vec![],
            // Our uniforms (we have none)
            uniforms: vec![]
        };

        // The three default verticies that will make up our triangle. If you want, this can be blank.
        let vertices = [Vertex::new(0.0, 0.5), Vertex::new(-0.5, -0.5), Vertex::new(0.5, -0.5)];
        // Create a vertex buffer, ours contains three verticies
        let vertex = painter.vertex_buffer::<Vertex>(&vertices, Some("Triangle vertex buffer"));

        // Creates an index buffer for our triangle. This indicates the order of which the verticies are to be assembled into triangles. We only have three, so the indicies here are 0, 1 and 2
        let indicies = painter.index_buffer(&[0, 1, 2], Some("Triangle index buffer"));

        Self {
            pipeline,
            indicies,
            vertices: vertex
        }
    }

    // This function is used to update our uniform buffer. As we don't have one, we return a blank vector
    fn prepare(&'a mut self, _context: Self::PrepareContext) -> Vec<(&'a mut UniformBuffer, Vec<Self::Uniforms>)> {
        vec![]
    }

    // This function is used to describe the layout of our pipeline
    fn description() -> PipelineDescription<'a> {
        PipelineDescription {
            vertex_layout: &[VertexFormat::Floatx2], // Layout of 2 floats
            pipeline_layout: None, // Has no bindings, so left empty
            shader: ShaderFile::Wgsl(include_str!("./shaders/triangle_square.wgsl")), // Takes in triangle shader
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

    // Get the size of the window
    let winsize = window.inner_size();

    // Get the preferred texture format for the s
    let pref_format = painter.preferred_format();

    // Configure the surface
    painter.configure(euclid::Size2D::new(winsize.width, winsize.height), wgpu::PresentMode::Fifo, pref_format);

    // A simple switch to keep track of whether it's a pentagon or a triangle
    let mut switch = false;

    // Create our pipeline :D
    let mut pipeline = painter.pipeline::<TrianglePipe>(Blending::default(), pref_format, Some("Triangle shader"));

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
                    // Check if the space key is pressed
                    WindowEvent::KeyboardInput { input: KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::Space), .. }, .. } => {
                        // Invert switch
                        switch = !switch;
                        println!("Switched");
                        if switch {
                            // Vertices for square
                            let vertex = [
                                Vertex::new(-0.25, -0.25),
                                Vertex::new(-0.25, 0.25),
                                Vertex::new(0.25, 0.25),
                                Vertex::new(0.25, -0.25)
                            ];
                            // Indicies for square
                            let indicies = vec![
                                0, 1, 2,
                                2, 3, 0
                            ];

                            // Replace buffer with new bigger one if created
                            if let Some(updated_buff) = painter.update_vertex_buffer(&vertex, &mut pipeline.vertices) {
                                pipeline.vertices = updated_buff;
                            }

                            // Replace buffer with new bigger one if created
                            if let Some(updated_buff) = painter.update_index_buffer(indicies, &mut pipeline.indicies) {
                                pipeline.indicies = updated_buff;
                            }
                        } else {
                            // Vertices for triangle
                            let vertex = [Vertex::new(0.0, 0.5), Vertex::new(-0.5, -0.5), Vertex::new(0.5, -0.5)];

                            // Indicies for a triangle
                            let indicies = vec![0, 1, 2];

                            // Replace buffer with new bigger one if created
                            if let Some(updated_buff) = painter.update_vertex_buffer(&vertex, &mut pipeline.vertices) {
                                pipeline.vertices = updated_buff;
                            }

                            // Replace buffer with new bigger one if created
                            if let Some(updated_buff) = painter.update_index_buffer(indicies, &mut pipeline.indicies) {
                                pipeline.indicies = updated_buff;
                            }
                        }
                        window.request_redraw();
                    }
                    _ => ()
                }
                
            },
            Event::RedrawRequested(_) => {
                // Time to draw our shape :D

                // Create a frame. This represents our, well, frame
                let mut frame = painter.frame();

                // Grab the current surface
                let current_surface = painter.current_frame().unwrap();

                {
                    // Initiate a render pass
                    let mut pass = frame.pass(PassOp::Clear(parrot::color::Rgba::new(0.1, 0.2, 0.3, 1.0)), &current_surface, None);

                    // Set our vertex buffer
                    pass.set_parrot_vertex_buffer(&pipeline.vertices);

                    // Set our index buffer
                    pass.set_parrot_index_buffer(&pipeline.indicies);

                    // Set our pipeline
                    pass.set_parrot_pipeline(&pipeline);

                    // Perform the render pass on the entire vertex buffer
                    pass.draw_parrot_indexed(0..pipeline.indicies.size, 0..1)
                }

                // Present our frame
                painter.present(frame);
            }
            _ => ()
        }
    });
}