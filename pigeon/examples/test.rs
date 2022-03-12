// use winit::{event::WindowEvent, window::Window, event_loop::ControlFlow};

// struct DebugEventSystem;

// impl pigeon::EventSystem for DebugEventSystem {
//     fn event(&self, event: &WindowEvent, window: &Window, control_flow: &mut ControlFlow) {
        
//     }
// }


fn main() {
    let p = pigeon::Pigeon::new(log::LevelFilter::max(), "main".to_string());
    p.run_event(pigeon::event_system::DebugEventSystem);
}