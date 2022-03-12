use winit::{
    event::{WindowEvent, ElementState, KeyboardInput, VirtualKeyCode},
    window::Window,
    event_loop::ControlFlow,
};

/// A trait that gets [`winit::event::WindowEvent`] from the event loop
pub trait EventSystem {
    fn event(&self, event: &WindowEvent, window: &Window, control_flow: &mut ControlFlow);
}

/// A debuging event system that quits when q is pressed
pub struct DebugEventSystem;

impl EventSystem for DebugEventSystem {
    fn event(&self, event: &WindowEvent, window: &Window, control_flow: &mut ControlFlow) {
        if let WindowEvent::KeyboardInput{
            device_id: _,
            input: KeyboardInput {
                scancode: _,
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Q),
                modifiers: _
            },
            is_synthetic: _ } = event {
            *control_flow = ControlFlow::Exit;
        } else {
            window.request_redraw();
        }
    }
}