extern crate pigeon_parrot as parrot;
use std::{ops::Deref};
use parrot::{
    pipeline::{Plumber,
        PipelineDescription,
        PipelineCore,
        Pipeline, Blending, PipelineLayout,
    },
    buffers::{UniformBuffer, VertexBuffer},
    vertex::{VertexFormat, VertexLayout},
    shader::ShaderFile,
    painter::PassOp, Painter,
    device::Device, RenderPassExtention,
};
use wgpu::TextureUsages;
use winit::event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
use winit::event_loop::ControlFlow;
use euclid::Size2D;

/// =======================================================================
/// I recommend reading [learn wgpu](https://sotrh.github.io/learn-wgpu/)
/// before using parrot and reading this example. I also recommend reading
/// the msaa-line example in wgpu
/// 
/// This example is intended to show multisampled lines.
/// =======================================================================

/// The struct that will represent our verticies. As we are only drawing 2D shapes, we have a position composed of 2 floats
/// The vertices must implement [`bytemuck::Pod`], [`bytemuck::Zeroable`] and [`Copy`]
/// They also must be #[repr(C)]
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    // Create a new vertex
    pub fn new(x: f32, y: f32, r: f32, g: f32, b: f32) -> Self {
        Self {
            position: [x,y],
            color: [r,g,b],
        }
    }
}

// The pipeline for our lines
pub struct LinePipe {
    pipeline: PipelineCore,
    vertices: VertexBuffer,
    vert_amount: u32,
}
impl Deref for LinePipe {
    type Target = PipelineCore;

    /// This function returns our [`PipelineCore`]. Normally we would create this, but as this is a simple line pipeline, I won't bother
    fn deref(&self) -> &Self::Target {
        &self.pipeline
    }
}

impl<'a> Plumber<'a> for LinePipe {
    /// Typically contains the information required to update our uniform buffer. We don't have one so we use ()
    type PrepareContext = ();
    /// Once again we have no uniforms, so we use ()
    type Uniforms = ();

    /// This is the function that will be used to create our pipeline
    fn setup(pipe: Pipeline, painter: &Painter) -> Self {
        let pipeline = PipelineCore {
            // The actual pipeline
            pipeline: pipe,
            // Our bindings (we have none)
            bindings: vec![],
            // Our uniforms (we have none)
            uniforms: vec![]
        };

        let mut vertices = vec![];

        let max = 50;
        for i in 0..max {
            let percent = i as f32 / max as f32;
            let (sin, cos) = (percent * 2.0 * std::f32::consts::PI).sin_cos();
            vertices.push(Vertex::new(0.0, 0.0, 1.0, -sin, cos));
            vertices.push(Vertex::new(1.0 * cos, 1.0 * sin, cos, -sin, 1.0));
        }
        // Create a vertex buffer, ours contains three verticies
        let vertex = painter.vertex_buffer::<Vertex>(&vertices, Some("Line vertex buffer"));

        Self {
            pipeline,
            vertices: vertex,
            vert_amount: max * 2,
        }
    }

    // This function is used to update our uniform buffer. As we don't have one, we return a blank vector
    fn prepare(&'a mut self, _context: Self::PrepareContext, _: &mut Painter) -> Vec<(&'a mut UniformBuffer, Vec<Self::Uniforms>)> {
        vec![]
    }

    // This function is used to describe the layout of our pipeline
    fn description() -> PipelineDescription<'a> {
        PipelineDescription {
            vertex_layout: &[VertexFormat::Floatx2, VertexFormat::Floatx3], // Layout of 2 floats for position, 3 floats for color
            pipeline_layout: None, // Has no bindings, so left empty
            shader: ShaderFile::Wgsl(include_str!("./shaders/multisampled_line.wgsl")), // Takes in line shader
            name: Some("Line pipeline") // Name of pipeline
        }
    }
}

fn main() {
    // Initialise the logging output at info level only from parrot
    env_logger::builder().filter_level(log::LevelFilter::max()).init();
    
    // Create an event loop
    let event_loop = winit::event_loop::EventLoop::new();
    // Create a window to draw to
    let window = winit::window::WindowBuilder::new().with_title("msaa lines :D").build(&event_loop).unwrap();

    // Create a wgpu instance
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };

    // A variable to hold the samples for our example
    let mut samples = 4;

    // Create the painter
    let mut painter = pollster::block_on(parrot::Painter::for_surface(surface, &instance, samples)).unwrap();

    // Get the size of the window
    let winsize = window.inner_size();

    // Configure the surface
    painter.configure(Size2D::new(winsize.width, winsize.height), wgpu::PresentMode::Fifo, wgpu::TextureFormat::Bgra8UnormSrgb);

    // Create our pipeline. As we are using lines instead of triangles as our primative geometry, we have to create a custom pipeline.
    // As we are passing in a function and not a closure, we must provide both the function type and pipeline type. The function type we need is already in parrot
    // As parrot will hand us the values it works with, if you want another sample you can see how pipeline is
    // created in device
    let mut pipeline = painter.custom_pipeline::<LinePipe, parrot::painter::PipelineFunction>(Some("Line shader"), create_pipeline);

    // Create the multisampled framebuffer
    let mut multisample = painter.texture(Size2D::new(winsize.width, winsize.height), wgpu::TextureFormat::Bgra8UnormSrgb, TextureUsages::RENDER_ATTACHMENT, Some("Multisampled framebuffer"), true);

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
                        multisample = painter.texture(size, wgpu::TextureFormat::Bgra8UnormSrgb, TextureUsages::RENDER_ATTACHMENT, Some("Multisampled framebuffer"), true);
                        painter.configure(size, wgpu::PresentMode::Fifo, wgpu::TextureFormat::Bgra8UnormSrgb)
                    },
                    WindowEvent::KeyboardInput { input: KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::Space), .. }, .. } => {
                        // Switch multisampling
                        if samples == 1 {
                            samples = 4;
                        } else {
                            samples = 1;
                        }

                        // Update the painters multisample variable
                        painter.update_sample_count(samples);
                        // Update the pipeline
                        pipeline = painter.custom_pipeline::<LinePipe, parrot::painter::PipelineFunction>(Some("Line shader"), create_pipeline);
                        // Update the multisample texture
                        multisample = painter.texture(painter.size(), wgpu::TextureFormat::Bgra8UnormSrgb, TextureUsages::RENDER_ATTACHMENT, Some("Multisampled framebuffer"), true);

                        window.request_redraw();
                    }
                    _ => ()
                }
            },
            Event::RedrawRequested(_) => {
                // Time to draw our lines

                // Create a frame. This represents our, well, frame
                let mut frame = painter.frame();

                // Grab the current surface, we grab the one with no depth buffer attached
                let current_surface = painter.current_frame_no_depth().unwrap();

                {
                    let mut pass: wgpu::RenderPass;
                    // Initiate a render pass
                    if samples == 4 {
                        pass = frame.pass(PassOp::Clear(parrot::color::Rgba::new(0.0, 0.0, 0.0, 1.0)), &current_surface, Some(&multisample.view));
                    } else {
                        pass = frame.pass(PassOp::Clear(parrot::color::Rgba::new(0.0, 0.0, 0.0, 1.0)), &current_surface, None);
                    }

                    // Set our pipeline
                    pass.set_parrot_pipeline(&pipeline);
                    pass.draw_buffer_range(&pipeline.vertices, 0..pipeline.vert_amount);
                    // pass.execute_bundles(std::iter::once(&rb));
                }

                // Present our frame
                painter.present(frame);
            }
            _ => ()
        }
    });
}

/// Function used to create our custom [`wgpu::RenderPipeline`] and then our [`Pipeline`]
fn create_pipeline(device: &Device, pipeline_layout: PipelineLayout, vertex_layout: VertexLayout, shader: wgpu::ShaderModule, multisample: wgpu::MultisampleState, name: Option<&str>) -> Pipeline {
    // Get the vertex attributes
    let vertex_attrs = vertex_layout.to_wgpu();

    // Grab the binding group layouts
    let mut b_layouts = Vec::new();

    for s in pipeline_layout.b_layouts.iter() {
        b_layouts.push(&s.wgpu);
    }

    // Construct the pipeline layout
    let layout = &device.wgpu.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: name,
        bind_group_layouts: b_layouts.as_slice(),
        push_constant_ranges: &[]
    });
    
    // Blending
    let (src_factor, dst_factor, operation) = Blending::default().as_wgpu();
    let targets = [Some(wgpu::ColorTargetState {
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor,
                dst_factor,
                operation
            },
            alpha: wgpu::BlendComponent {
                src_factor,
                dst_factor,
                operation,
            },
        }),
        write_mask: wgpu::ColorWrites::ALL,
    })];

    // Create the wgpu pipeline
    let desc = wgpu::RenderPipelineDescriptor {
        label: name,
        vertex: wgpu::VertexState { module: &shader, entry_point: "vs_main", buffers: &[vertex_attrs] },
        layout: Some(layout),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            ..Default::default()
        },
        depth_stencil: None,
        multisample,
        multiview: None,
        fragment: Some(wgpu::FragmentState{ module: &shader, entry_point: "fs_main", targets: &targets})
    };

    let wgpu = device.wgpu.create_render_pipeline(&desc);

    // Our pipeline
    Pipeline { wgpu, layout: pipeline_layout, vertex_layout: vertex_layout }
}