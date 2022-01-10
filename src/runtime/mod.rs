use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::Window;
use crate::looputil::TimerStatus;
use crate::runtime::program::{Program, RendererTexture};
use crate::State;

pub mod program;

pub fn start<Shared: 'static,Proxy>(window: Window, event_loop: EventLoop<Proxy>, mut state: State, mut global: Shared, programs: &mut Vec<Box<dyn Program<Shared = Shared, Proxy = Proxy>>>) {

    let mut progs_rends = vec![];

    // run program inits and fetch renderers
    while !programs.is_empty() {
        let mut prog = programs.pop().unwrap();
        let mut rend = prog.init(&mut global, &state);
        progs_rends.push((prog, rend));
    }

    event_loop.run(move |event ,target ,control_flow|{
        match event {
            Event::MainEventsCleared => {
                // TODO: create a fill and drain queue instead of iterating on every event
                for (prog, rend) in &mut progs_rends {
                    match rend.update_timing.check() {
                        TimerStatus::Ready => {
                            prog.update(&mut global, &state, rend);
                            rend.update_timing.reset();
                        }
                        TimerStatus::Waiting(_) => {}
                        TimerStatus::Ignore => {}
                    }
                }
            }
            Event::RedrawRequested(wid) => {

                let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
                    label: None
                });

                let surface_tex = state.surface.get_current_texture().unwrap();
                let surface_view = surface_tex.texture.create_view(&wgpu::TextureViewDescriptor{
                    label: None,
                    format: Some(state.config.format.clone()),
                    .. Default::default()
                });

                for (prog, rend) in &mut progs_rends {

                    {
                        // TODO: allow more renderpass customization
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                            label: None,
                            color_attachments: &[
                                wgpu::RenderPassColorAttachment{
                                    view: match &rend.texture{
                                        RendererTexture::Surface => &surface_view,
                                        RendererTexture::Texture(tex) => {
                                            &tex.view
                                        }
                                    },
                                    resolve_target: None,
                                    ops: wgpu::Operations{
                                        load: rend.loadop.clone(),
                                        store: true
                                    }
                                }
                            ],
                            depth_stencil_attachment: None
                        });

                        match rend.render_timing.check() {
                            TimerStatus::Ready => {
                                prog.render(&mut global, &state, &mut rpass);
                                rend.render_timing.reset();
                            }
                            TimerStatus::Waiting(_) => {}
                            TimerStatus::Ignore => {}
                        };

                    }
                }

                state.queue.submit(std::iter::once(encoder.finish()));
                surface_tex.present();
            }
            e => {
                for (prog, rend) in &mut progs_rends {
                    prog.on_event(&mut global, &mut state, rend, &e);
                }
            },
        }

    });
}