/// A remake of the hello-triangle example from the `wgpu` repository.

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
use crate::shader::TrianglePipe;

async fn run(event_loop: EventLoop<()>, window: Window) {

    let mut state = wgpu_quick::State::new(&window, wgpu::Backends::all()).await;

    let triangle_pipe = make_pipline::<TrianglePipe>(&state, &[], &[]);

    let triangle_obj = RenderObject{
        pipeline: Arc::clone(&triangle_pipe.pipeline),
        bind_groups: vec![],
        model: DrawInput::NonIndexed {
            verticies: 0..3,
            instances: 0..1
        }
    };

    event_loop.run(move |event, _, control_flow| {

        // Referencing `state` in this closure moves the ownership.

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Reconfigure the surface with the new size
                state.resize(size);
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
                    triangle_obj.render_this(&mut rpass);
                }

                state.queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    wgpu_quick::init::start_wgpu!(window, event_loop);
}