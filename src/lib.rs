use winit::dpi::PhysicalSize;

pub mod bindings;
pub mod renderobj;
pub mod texture;
pub mod rect;
pub mod model;
pub mod looputil;
pub mod uniforms;
pub mod runtime;
pub mod utils;
pub mod pipelines;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct State {
    pub scalefactor: f64,
    pub instance: wgpu::Instance,
    pub config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl State {
    /// Initialize a `wgpu` state.\
    /// TODO: allow setting specific adapter parameters.
    pub async fn new(window: &winit::window::Window, backend: wgpu::Backends) -> Self {
        let size = window.inner_size();
        let scalefactor = window.scale_factor();
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

       let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        surface.configure(&device, &config);

        Self {
            scalefactor,
            instance,
            config,
            surface,
            adapter,
            device,
            queue
        }

    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        // self.config.width = (size.width as f64 * self.scalefactor) as u32;
        // self.config.height = (size.height as f64 * self.scalefactor) as u32;
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }


}