mod shader;

use std::borrow::Cow;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use wgpu_quick::pipelines::{Pipeline, VertexDesc, FragmentDesc, make_pipline};
use wgpu_quick::renderobj::{RenderObject, DrawInput};
use std::sync::Arc;
use crate::shader::MouseTexPipeline;
use wgpu_quick::texture::Texture;
use wgpu_quick::bindings::{Bindings, Binder};
use wgpu_quick::uniforms::Uniform;
use wgpu_quick::looputil::{Timing, TimerStatus};
use std::time::Instant;

async fn run(event_loop: EventLoop<()>, window: Window) {

    let mut state = wgpu_quick::State::new(&window, wgpu::Backends::all()).await;

    let texture = Texture::from_bytes(&state.device, &state.queue, include_bytes!("kermit.png")).expect("Could not load texture");

    let mut mouse_pos = Uniform::new(&state.device, [0.0,0.0]);

    let bindings = Bindings::make(&state.device, vec![
       Binder {
           binding: 0,
           visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
           ty: wgpu::BindingType::Buffer {
               ty: wgpu::BufferBindingType::Uniform,
               has_dynamic_offset: false,
               min_binding_size: None
           },
           count: None,
           resources: vec![
               mouse_pos.get_buffer().as_entire_binding()
           ]
       },
        Binder {
            binding: 1,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float {
                    filterable: true
                },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false
            },
            count: None,
            resources: vec![
                wgpu::BindingResource::TextureView(&texture.view)
            ]
        },
        Binder {
            binding: 2,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
            resources: vec![
                wgpu::BindingResource::Sampler(&texture.sampler)
            ]
        }
    ]);

    let mousetex_pipe= make_pipline::<MouseTexPipeline>(&state, &[&bindings.bind_layout], &[]);

    let mousetex_obj = RenderObject{
        pipeline: Arc::clone(&mousetex_pipe.pipeline),
        bind_groups: vec![Arc::clone(&bindings.bind_groups[0])],
        model: DrawInput::NonIndexed {
            verticies: 0..3,
            instances: 0..1
        }
    };

    let mut framerate = Timing::Framerate { last_called_at: Instant::now(), desired_framerate: 60.0 };

    event_loop.run(move |event, _, control_flow| {

        // Referencing `state` in this closure moves the ownership.

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::Resized(size) => {
                        state.resize(size);
                    }
                    WindowEvent::Moved(_) => {}
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    WindowEvent::Destroyed => {}
                    WindowEvent::DroppedFile(_) => {}
                    WindowEvent::HoveredFile(_) => {}
                    WindowEvent::HoveredFileCancelled => {}
                    WindowEvent::ReceivedCharacter(_) => {}
                    WindowEvent::Focused(_) => {}
                    WindowEvent::KeyboardInput { .. } => {}
                    WindowEvent::ModifiersChanged(_) => {}
                    WindowEvent::CursorMoved { device_id, position, modifiers } => {
                        (*mouse_pos)[0] = (position.x as f32)/(state.config.width as f32);
                        (*mouse_pos)[1] = (position.y as f32)/(state.config.height as f32);

                        match framerate.check() {
                            TimerStatus::Ready => {
                                framerate.reset();
                                mouse_pos.sync(&state.queue);
                                window.request_redraw();
                            }
                            TimerStatus::Waiting(_) => {}
                            TimerStatus::Ignore => {}
                        }
                    }
                    WindowEvent::CursorEntered { .. } => {}
                    WindowEvent::CursorLeft { .. } => {}
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::MouseInput { .. } => {}
                    WindowEvent::TouchpadPressure { .. } => {}
                    WindowEvent::AxisMotion { .. } => {}
                    WindowEvent::Touch(_) => {}
                    WindowEvent::ScaleFactorChanged { .. } => {}
                    WindowEvent::ThemeChanged(_) => {}
                }
            }
            Event::RedrawRequested(_) => {
                let frame = state.surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    mousetex_obj.render_this(&mut rpass);
                }

                state.queue.submit(Some(encoder.finish()));
                frame.present();
            }
            _ => {}
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    #[cfg(not(target_arch = "wasm32"))]
        {
            // Temporarily avoid srgb formats for the swapchain on the web
            pollster::block_on(run(event_loop, window));
        }
    #[cfg(target_arch = "wasm32")]
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
            use winit::platform::web::WindowExtWebSys;
            // On wasm, append the canvas to the document body
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
            wasm_bindgen_futures::spawn_local(run(event_loop, window));
        }
}
