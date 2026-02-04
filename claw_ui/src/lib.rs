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

        println!("ClawBrowser: Window initialized. Starting event loop...");

        let _ = event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        // Rendering logic here
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
