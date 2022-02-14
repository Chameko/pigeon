use winit::{
    event::WindowEvent,
    window::Window,
    event_loop::ControlFlow,
};

/// A trait that gets [`winit::event::WindowEvent`] from the event loop
pub trait EventSystem {
    fn event(&self, event: &WindowEvent, window: &Window, control_flow: &ControlFlow);
}