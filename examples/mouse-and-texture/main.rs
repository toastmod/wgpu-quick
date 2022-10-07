mod shader;

use std::borrow::Cow;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use wgpu_quick::{pipelines::{Pipeline, VertexDesc, FragmentDesc, make_pipline}};
use wgpu_quick::renderobj::{RenderObject, DrawInput};
use std::sync::Arc;
use crate::shader::MouseTexPipeline;
use wgpu_quick::texture::Texture;
use wgpu_quick::bindings::{Bindings, Binder};
use wgpu_quick::looputil::{Timing, TimerStatus};
use std::time::Instant;
use wgpu_quick::buffer::uniform::Uniform;

async fn run(event_loop: EventLoop<()>, window: Window) {

    // Initialize wgpu state for any backend
    let mut state = wgpu_quick::State::new_winit(&window, None, wgpu::Backends::all()).await;

    // Load a texture from an image file.
    let texture = Texture::from_bytes(&state.device, &state.queue, include_bytes!("kermit.png")).expect("Could not load texture");

    // Create a uniform variable to represent mouse position.
    let mut mouse_pos = Uniform::new(&state.device, [0.0,0.0]);

    // Create a set of bindings for mouse position, texture view, and sampler.
    // Binders must all have the same number of resources or else the program will panic.
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

    // Load a pipeline that uses the binding's layout.
    let mousetex_pipe= make_pipline::<MouseTexPipeline>(&state, &[&bindings.bind_layout], &[]);

    // Create a render object that uses the pipeline with our compatible binding.
    let mousetex_obj = RenderObject{
        pipeline: Arc::clone(&mousetex_pipe.pipeline),
        bind_groups: vec![Arc::clone(&bindings.bind_groups[0])],
        model: DrawInput::NonIndexed {
            verticies: 0..3,
            instances: 0..1
        }
    };

    // Set a framerate.
    let mut framerate = Timing::framerate(60.0);

    event_loop.run(move |event, _, control_flow| {

        // Referencing `state` in this closure moves the ownership.

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::Resized(size) => {
                        // Change configuration on resize.
                        state.resize(size);
                    }

                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }

                    WindowEvent::CursorMoved { device_id, position, modifiers } => {

                        // Calculate mouse position in screen units
                        (*mouse_pos)[0] = (position.x as f32)/(state.config.width as f32);
                        (*mouse_pos)[1] = (position.y as f32)/(state.config.height as f32);

                        // Check if it is time to render a frame.
                        match framerate.check() {
                            TimerStatus::Ready => {
                                // Reset the framerate timer.
                                framerate.reset();
                                // Sync the uniform variable with the local memory.
                                mouse_pos.sync(&state.queue);
                                //  Request a redraw event.
                                window.request_redraw();
                            }
                            TimerStatus::Waiting(_) => {}
                            TimerStatus::Ignore => {}
                        }
                    }

                    _=>{}
                }
            }
            Event::RedrawRequested(_) => {
                // Get the surface texture and create a render pass.
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
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    // Write from the RenderObject to the RenderPass.
                    mousetex_obj.render_this(&mut rpass);
                }

                // Submit and present.
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
    // Start wgpu for your platform. 
    wgpu_quick::init::start_wgpu!(window, event_loop, run);
}
