/// A remake of the hello-triangle example from the `wgpu` repository.

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
use crate::shader::TrianglePipe;

async fn run(event_loop: EventLoop<()>, window: &Window) {

    // Initialize wgpu for any backend.
    let mut state = wgpu_quick::State::new_winit(window, None, wgpu::Backends::all()).await;

    // Create a new pipeline instance.
    let triangle_pipe = make_pipline::<TrianglePipe>(&state, &[], &[]);

    // Make a RenderObject that uses this pipeline.
    let triangle_obj = RenderObject{
        pipeline: Arc::clone(&triangle_pipe.pipeline),
        bind_groups: vec![],
        model: DrawInput::NonIndexed {
            verticies: 0..3,
            instances: 0..1
        }
    };

    // Begin the event loop.
    event_loop.run(move |event, _, control_flow| {

        // Referencing `state` in this closure moves the ownership.

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Reconfigure the surface with the new size.
                state.resize(size);
            }

            // Only render on redraw request events.
            Event::RedrawRequested(_) => {
                state.quick_render_pass(None, wgpu::Color::GREEN, &mut |rpass|{
                    triangle_obj.render_this(rpass);
                });
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
    // Start wgpu for your platform.
    wgpu_quick::init::start_wgpu!(&window, event_loop, run);
}