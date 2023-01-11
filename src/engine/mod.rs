use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use self::{renderer::RendererState, resource::ResourceManager};

pub mod renderer;
pub mod resource;
pub mod world;

pub struct EngineResources {
    renderer: RendererState,
    resource_manager: ResourceManager,
}

pub async fn start() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("RGraphics")
        .build(&event_loop)
        .unwrap();

    let renderer = RendererState::new(&window).await;
    let resource_manager = ResourceManager::new("./res", &renderer);

    let mut resources = EngineResources {
        renderer,
        resource_manager,
    };

    event_loop.run(move |event, _, flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::Resized(size) => {
                    resources.renderer.resize(size);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                let surface = resources.renderer.surface.get_current_texture().unwrap();

                

                surface.present();
            }
            _ => (),
        };
    });
}
