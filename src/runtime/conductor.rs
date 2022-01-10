use std::ops::DerefMut;
use std::time::Duration;
use crate::looputil::{TimerStatus, Timing};
use crate::State;
use crate::runtime::program::*;
use crate::utils::{Initable, InnerInit};

pub(crate) type InputRoutingFunc<Shared,Proxy> = fn(&mut Option<Box<Shared>>, winit::event::Event<Proxy>) -> ();

pub struct Conductor<Shared,Proxy: 'static> {
    program_renderers: Vec<Option<Box<ProgRenderer>>>,
    global: Option<Box<Shared>>,
    programs: Vec<Option<Box<dyn Program<Shared = Shared,Proxy = Proxy>>>>,
    routef: Option<Box<InputRoutingFunc<Shared,Proxy>>>
}

impl<Shared,Proxy: 'static> Conductor<Shared,Proxy> {
    pub fn new(global: Box<Shared>) -> Self {
        Self {
            program_renderers: vec![],
            global: Some(global),
            programs: vec![],
            routef: None
        }
    }

    pub fn set_global_ptr(&mut self, global: Box<Shared>) -> Option<Box<Shared>> {
        std::mem::replace(&mut self.global, Some(global) )
    }

    pub(crate) fn process_input(&mut self, event: winit::event::Event<Proxy>) {
        let mut global = std::mem::replace(&mut self.global, None);
        // TODO: make routef return an enum that sends the event to a specific program
        //  figure out how to indentify programs
        (self.routef.as_ref().unwrap())(&mut global, event);
        std::mem::swap(&mut self.global, &mut global);
    }

    pub fn set_input_routing(&mut self, route: Box<InputRoutingFunc<Shared,Proxy>>) -> Option<Box<InputRoutingFunc<Shared,Proxy>>>  {
        std::mem::replace(&mut self.routef, Some(route))
    }

    pub(crate) fn new_program(&mut self, program: Box<dyn Program<Shared = Shared, Proxy = Proxy>>) -> usize {
        let id = self.programs.len();
        self.programs.push(Some(program));
        id
    }


    pub(crate) fn cycle_programs(&mut self, state: &mut State, ctrl: &mut winit::event_loop::ControlFlow) {

        //create encoder
        let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: None
        });
        // create surface texture view
        let surface_tex = state.surface.get_current_texture().expect("could not get surface texture");
        let surface_view = surface_tex.texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut global = {std::mem::replace(&mut self.global, None)};

            let mut wait_time: InnerInit<Duration> = InnerInit::new(None);

            for i in 0..self.programs.len() {
                match &mut self.program_renderers[i] {
                    None => println!("Program {} not available!",i),
                    Some(renderer) => {

                        match renderer.render_timing.check() {
                            TimerStatus::Ready => {

                                let tview = match &renderer.texture {
                                    RendererTexture::Surface => {
                                        &surface_view
                                    }
                                    RendererTexture::Texture(tex) => {
                                        &tex.view
                                    }
                                };

                                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                                    label: None,
                                    color_attachments: &[
                                        wgpu::RenderPassColorAttachment {
                                            view: &tview,
                                            resolve_target: None,
                                            ops: wgpu::Operations {
                                                load: renderer.loadop,
                                                store: true
                                            }
                                        }
                                    ],
                                    depth_stencil_attachment: None
                                });

                                self.programs[i].as_mut().unwrap().render(global.as_mut().unwrap(), &state, &mut render_pass);
                            }
                            TimerStatus::Waiting(wait) => {
                                // TODO: wait to execute specific programs
                                //  instead of cycling through all every time the event loop is triggered

                                match wait_time.match_me() {
                                    Initable::Null(shell) => {
                                        *shell = Some(wait);
                                    }
                                    Initable::Active(global_wait) => {
                                        if wait < *global_wait {
                                            *global_wait = wait;
                                        }
                                    }
                                }

                            }
                            TimerStatus::Ignore => {}
                        };

                          match renderer.update_timing.check() {
                            TimerStatus::Ready => {
                                self.programs[i].as_mut().unwrap().update(global.as_mut().unwrap(), &state, renderer.as_mut());
                            }
                            TimerStatus::Waiting(wait) => {

                                match wait_time.match_me() {
                                    Initable::Null(shell) => {
                                        *shell = Some(wait);
                                    }
                                    Initable::Active(global_wait) => {
                                        if wait < *global_wait {
                                            *global_wait = wait;
                                        }
                                    }
                                }
                            }
                            TimerStatus::Ignore => {}
                        }

                    }
                }
            }

            std::mem::swap(&mut self.global, &mut global)

        }

        state.queue.submit(std::iter::once(encoder.finish()));
        surface_tex.present();
    }
}