/// A remake of the hello-triangle example from the `wgpu` repository.

mod shader;

use std::borrow::Cow;
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window, dpi::PhysicalSize,
};
use wgpu_quick::{pipelines::{Pipeline, VertexDesc, FragmentDesc, make_pipline}, Backends, State};
use wgpu_quick::renderobj::{RenderObject, DrawInput};
use std::sync::Arc;
use crate::shader::TrianglePipe;

async fn run() {

    // Use the native platform to somehow get a raw window handle here...
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let handle = window.raw_window_handle();
    let size = (window.inner_size().width,window.inner_size().height);
    let scalefactor = window.scale_factor();

    // Initialize wgpu for any backend, passing in the raw window handle and it's known display proportions.
    let mut state = State::new_raw(handle, size, scalefactor, None, Backends::ALL).await.expect("Could not create wgpu surface!");

    // Create a new pipeline instance.
    let triangle_pipe = make_pipline::<TrianglePipe>(&state, &[], &[]);

    // Make a RenderObject that uses this pipeline.
    let triangle_obj = RenderObject{
        pipeline: Arc::clone(&triangle_pipe.pipeline),
        bind_groups: vec![],
        model: DrawInput::NonIndexed {
            vertices: 0..3,
            instances: 0..1
        }
    };

    // Fetch the surface texture. 
    let frame = state.surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");

    // Create a view from the surface texture.
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    // Begin a render pass with a command encoder.
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

        // Let the RenderObject write to the RenderPass
        triangle_obj.render_this(&mut rpass);
    }

    // Finalize and submit the commands, present the frame.
    state.queue.submit(Some(encoder.finish()));
    frame.present();


    // In this case you'd continue to use the native platform's event handler, but we will just use winit here. 
    event_loop.run(move |e, _, ctrl|{
        match e {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::Resized(s) => {
                    state.resize(s);
                },
                WindowEvent::CloseRequested => {
                    *ctrl = ControlFlow::Exit
                },
                _ => {}
            },

            _ => {}

        }
    })

}

fn main() {
    // Start wgpu for your platform.
    wgpu_quick::init::start_wgpu_raw!(run);
}