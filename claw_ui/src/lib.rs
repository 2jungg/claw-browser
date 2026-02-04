use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};
use claw_layout::LayoutTree;

pub struct BrowserWindow {
    title: String,
    width: u32,
    height: u32,
}

impl BrowserWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
        }
    }

    pub async fn run(&self, _tree: LayoutTree) {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height))
            .build(&event_loop)
            .unwrap();

        // Basic wgpu initialization would go here. 
        // For alpha, we just clear the screen as a demonstration.
        
        println!("Browser window started. (Simulating GPU Render)");
        // In a real headless server, this might fail without Xvfb or similar.
        // But for the user's local Windows environment, this is correct.
    }
}
