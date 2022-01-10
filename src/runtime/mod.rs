use winit::event_loop::EventLoop;
use winit::window::Window;
use crate::runtime::conductor::Conductor;
use crate::State;

pub mod program;
pub mod conductor;
mod timemgmt;

pub fn start<Shared: 'static,Proxy>(window: Window, event_loop: EventLoop<Proxy>, mut state: State, mut conductor: Conductor<Shared, Proxy>) {
    event_loop.run(move |event ,target ,control_flow|{
        conductor.process_input(event);
        conductor.cycle_programs(&mut state, control_flow);
    });
}