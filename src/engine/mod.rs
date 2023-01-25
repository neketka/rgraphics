use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use self::{renderer::RendererState, resource::ResourceManager, world::World};

pub mod renderer;
pub mod resource;
pub mod world;

pub struct EngineResources {
    renderer: RendererState,
    resource_manager: ResourceManager,
}

pub async fn start(mut world: World) {
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

    world.init(&mut resources);

    let mut last_frame = std::time::Instant::now() - std::time::Duration::from_millis(10);
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
                let this_frame = std::time::Instant::now();
                let dt = (this_frame - last_frame).as_secs_f32();

                let surface = resources.renderer.surface.get_current_texture().unwrap();
                world.run(dt, &mut resources);
                surface.present();

                last_frame = this_frame;
            }
            _ => (),
        };
    });
}
