use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use self::resource::ResourceManager;

mod renderer;
mod resource;

pub struct Engine {
    window: Window,
    renderer: renderer::Renderer,
    resource_manager: ResourceManager<'static>,
    event_loop: EventLoop<()>,
}

impl Engine {
    pub async fn new() -> Engine {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Hello world")
            .build(&event_loop)
            .unwrap();
        let renderer = renderer::Renderer::new(&window).await;

        Engine {
            window,
            event_loop,
            renderer,
            resource_manager: ResourceManager::new("./res"),
        }
    }

    pub fn start(self) {
        self.event_loop.run(|event, _, flow| {
            match event {
                Event::WindowEvent { event, .. } => {
                    if event == WindowEvent::CloseRequested {
                        *flow = ControlFlow::Exit;
                        return;
                    }
                }
                _ => (),
            };
        });
    }
}
