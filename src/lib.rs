use raw_window_handle::{RawDisplayHandle, UiKitDisplayHandle, AppKitDisplayHandle, OrbitalDisplayHandle, XcbDisplayHandle, WaylandDisplayHandle, DrmDisplayHandle, GbmDisplayHandle, WindowsDisplayHandle, WebDisplayHandle, AndroidDisplayHandle, HaikuDisplayHandle, XlibDisplayHandle};
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

struct RWH {
    handle: raw_window_handle::RawWindowHandle
}

unsafe impl raw_window_handle::HasRawWindowHandle for RWH {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.handle.clone()
    }
}


unsafe impl raw_window_handle::HasRawDisplayHandle for RWH {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        match self.handle {
                raw_window_handle::RawWindowHandle::UiKit(_) => RawDisplayHandle::UiKit(UiKitDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::AppKit(_) => RawDisplayHandle::AppKit(AppKitDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::Orbital(_) => RawDisplayHandle::Orbital(OrbitalDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::Xlib(_) => RawDisplayHandle::Xlib(XlibDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::Xcb(_) => RawDisplayHandle::Xcb(XcbDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::Wayland(_) => RawDisplayHandle::Wayland(WaylandDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::Drm(_) => RawDisplayHandle::Drm(DrmDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::Gbm(_) => RawDisplayHandle::Gbm(GbmDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::Win32(_) => RawDisplayHandle::Windows(WindowsDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::WinRt(_) => RawDisplayHandle::Windows(WindowsDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::Web(_) => RawDisplayHandle::Web(WebDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::AndroidNdk(_) => RawDisplayHandle::Android(AndroidDisplayHandle::empty()),
                raw_window_handle::RawWindowHandle::Haiku(_) => RawDisplayHandle::Haiku(HaikuDisplayHandle::empty()),
                _ => unimplemented!(),
        }
    }
}


pub enum Backends {
    ALL,
    VULKAN,
    GL,
    DX12(wgpu::Dx12Compiler),
    DX11,
    METAL,
    BROWSER_WEBGPU
}

impl Backends {
    fn gen_instance_desc(self) -> wgpu::InstanceDescriptor {
        wgpu::InstanceDescriptor {
            backends: match &self {
                Backends::ALL => wgpu::Backends::all(),
                Backends::VULKAN => wgpu::Backends::VULKAN,
                Backends::GL => wgpu::Backends::GL,
                Backends::DX12(_) => wgpu::Backends::DX12,
                Backends::DX11 => wgpu::Backends::DX11,
                Backends::METAL => wgpu::Backends::METAL,
                Backends::BROWSER_WEBGPU => wgpu::Backends::BROWSER_WEBGPU
            },
            dx12_shader_compiler: match self {
                Backends::DX12(compiler) => compiler,
                _ => wgpu::Dx12Compiler::default()
            }
        }
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

        let swapchain_capabilities= surface.get_capabilities(&adapter);

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
            None => swapchain_capabilities.present_modes[0],
        };


        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_capabilities.formats[0],
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
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
    pub async fn new_winit(window: &winit::window::Window, preferred_mode: Option<wgpu::PresentMode>, backend: Backends) -> Result<Self, wgpu::CreateSurfaceError> {
        
        let size = window.inner_size();
        let scalefactor = window.scale_factor();
        let instance = wgpu::Instance::new(backend.gen_instance_desc());
        let surface = unsafe { instance.create_surface(window) }?;

        Ok(Self::_new(instance, surface, size, scalefactor, preferred_mode).await)

    }

    /// Initialize a `wgpu` state from a raw window handle with window size and scale factor.\
    /// TODO: allow setting specific adapter parameters.
    pub async fn new_raw(handle: raw_window_handle::RawWindowHandle, win_size: (u32,u32), scalefactor: f64, preferred_mode: Option<wgpu::PresentMode>, backend: Backends) -> Result<Self, wgpu::CreateSurfaceError> {

        let size = PhysicalSize::new(win_size.0,win_size.1);
        let instance = wgpu::Instance::new(backend.gen_instance_desc());
        let surface = unsafe { instance.create_surface(&RWH {handle}) }?;
        
        Ok(Self::_new(instance, surface, size, scalefactor, preferred_mode).await)

    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        // self.config.width = (size.width as f64 * self.scalefactor) as u32;
        // self.config.height = (size.height as f64 * self.scalefactor) as u32;
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }    

    pub fn get_capabilities(&self) -> wgpu::SurfaceCapabilities {
        self.surface.get_capabilities(&self.adapter)
    }

}