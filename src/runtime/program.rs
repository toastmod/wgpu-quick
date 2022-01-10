use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::looputil::Timing;
use crate::State;
use crate::texture::Texture;

pub enum RendererTexture {
    Surface,
    Texture(Arc<Texture>)
}

pub enum TimingStatus {
    Ready,
    Waiting(Duration)
}

pub struct ProgRenderer {
    pub texture: RendererTexture,
    pub loadop: wgpu::LoadOp<wgpu::Color>,
    pub render_timing: Timing,
    pub update_timing: Timing,
}

impl ProgRenderer {

    pub fn set_update_rate(&mut self, timing: Timing) {
        self.update_timing = timing;
    }

    pub fn set_render_rate(&mut self, timing: Timing) {
        self.render_timing = timing;
    }

    pub fn set_clear(&mut self, color: wgpu::Color) {
        self.loadop = wgpu::LoadOp::Clear(color)
    }

    pub fn set_texture(&mut self, texture: RendererTexture) {
        self.texture = texture
    }

    pub(crate) fn get_wait_time(&self) -> Duration {
        unimplemented!()
    }
}

pub enum ProgReturn<'a> {
    None,
    Exit,
    Set(&'a [ProgSettings])
}

pub enum ProgSettings {
    PLACEHOLDER
}

/// A program that with a render texture
pub trait Program {

    type Shared;
    type Proxy;

    fn init(&mut self, global: &mut Self::Shared, state: &State) -> ProgRenderer {
        ProgRenderer{
            texture: RendererTexture::Surface,
            loadop: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            render_timing: Timing::Framerate { last_called_at: Instant::now(), desired_framerate: 30.0 },
            update_timing: Timing::Never,
        }
    }

    fn on_event(&mut self, global: &mut Self::Shared, state: &State, renderer: &mut ProgRenderer, event: &winit::event::Event<Self::Proxy>) -> ProgReturn {
        unimplemented!()
    }

    fn update(&mut self, global: &mut Self::Shared, state: &State, renderer: &mut ProgRenderer) -> ProgReturn {
        unimplemented!()
    }

    fn render<'a>(&mut self, global: &mut Self::Shared, state: &State, render_pass: &mut wgpu::RenderPass<'a>) {
        unimplemented!()
    }

    fn on_exit(&mut self, global: &mut Self::Shared, state: &State) {
        unimplemented!()
    }

}