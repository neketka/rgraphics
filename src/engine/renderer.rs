use wgpu::*;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub surface_config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Renderer {
        let size = window.inner_size();
        let instance = Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode: CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            surface_config: config,
            size,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;

        self.surface.configure(&self.device, &self.surface_config);
    }
}
