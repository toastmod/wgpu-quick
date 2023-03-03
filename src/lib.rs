use std::rc::Rc;

use raw_window_handle::HasRawWindowHandle;
use wgpu::{SurfaceTexture, TextureView};
use winit::{dpi::PhysicalSize};

pub use pollster;

pub mod bytemuck;
pub mod bindings;
pub mod renderobj;
pub mod texture;
pub mod rect;
pub mod model;
pub mod looputil;
pub mod buffer;
pub mod runtime;
pub mod utils;
#[macro_use]
pub mod init;
pub mod pipelines;
// pub mod init;

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

// pub enum WindowMethod {
//     Winit(winit::window::Window),
//     RawHandle{handle: raw_window_handle::RawWindowHandle, size: (u32,u32), scalefactor: f64}
// }

// impl WindowMethod {
//     pub fn from_raw_handle(handle: raw_window_handle::RawWindowHandle, size: (u32,u32), scalefactor: f64) -> Self {
//         Self::RawHandle { handle, size, scalefactor }
//     }
//     pub fn from_winit(window: winit::window::Window) -> Self {
//         Self::Winit(window)
//     }
// }

// unsafe impl HasRawWindowHandle for WindowMethod {
//     fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
//         match self {
//             WindowMethod::Winit(window) => {
//                 window.raw_window_handle()
//             },
//             WindowMethod::RawHandle { handle, size, scalefactor } => {
//                 handle.clone()
//             },
//         }
//     }
// }

struct RWH {
    handle: raw_window_handle::RawWindowHandle
}

unsafe impl raw_window_handle::HasRawWindowHandle for RWH {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.handle.clone()
    }
}

impl State {

    async fn _new(instance: wgpu::Instance, surface: wgpu::Surface, size: PhysicalSize<u32>, scalefactor: f64, preferred_mode: Option<wgpu::PresentMode>) -> Self {
        // request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let swapchain_format = surface.get_supported_formats(&adapter)[0];

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

        let present_mode = match preferred_mode {
            Some(p) => p,
            None => surface.get_supported_modes(&adapter)[0],
        };


        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode,
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

    /// Initialize a `wgpu` state from a winit window.\
    /// TODO: allow setting specific adapter parameters.
    pub async fn new_winit(window: &winit::window::Window, preferred_mode: Option<wgpu::PresentMode>, backend: wgpu::Backends) -> Self {

        let size = window.inner_size();
        let scalefactor = window.scale_factor();
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(window) };

        Self::_new(instance, surface, size, scalefactor, preferred_mode).await

    }

    /// Initialize a `wgpu` state from a raw window handle with window size and scale factor.\
    /// TODO: allow setting specific adapter parameters.
    pub async fn new_raw(handle: raw_window_handle::RawWindowHandle, win_size: (u32,u32), scalefactor: f64, preferred_mode: Option<wgpu::PresentMode>, backend: wgpu::Backends) -> Self {

        let size = PhysicalSize::new(win_size.0,win_size.1);
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(&RWH {handle}) };
        
        Self::_new(instance, surface, size, scalefactor, preferred_mode).await

    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        // self.config.width = (size.width as f64 * self.scalefactor) as u32;
        // self.config.height = (size.height as f64 * self.scalefactor) as u32;
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }    

}