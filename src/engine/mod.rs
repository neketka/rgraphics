use std::rc::Rc;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use self::{renderer::Renderer, resource::ResourceManager};

mod renderer;
mod resource;
mod system;

pub struct Engine<'a> {
    pub window: Window,
    pub renderer: Rc<Renderer>,
    pub resource_manager: ResourceManager<'a>,
    event_loop: EventLoop<()>,
}

impl<'a> Engine<'a> {
    pub async fn new() -> Engine<'a> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("RGraphics")
            .build(&event_loop)
            .unwrap();

        let renderer = Rc::new(Renderer::new(&window).await);
        let engine = Engine {
            resource_manager: ResourceManager::new("./res", renderer.clone()),
            renderer,
            window,
            event_loop,
        };

        engine
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
