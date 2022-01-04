use crate::rect::WorldPoint;
use crate::wgpustate::State;
use crate::modelbuffers::Model;
use wgpu::Buffer;
use crate::State;
use crate::model::Model;
use std::ops::Range;

pub enum DrawInput {
    Model{my_model: Model, instances: Range<u32> },
    NonIndexed{ verticies: Range<u32>, instances: Range<u32>},
}

/// Data for a renderable object.
pub struct RenderObject<'a> {
    pub pipeline: &'a wgpu::RenderPipeline,
    pub layout_group: u32,
    pub bind_group: &'a wgpu::BindGroup,
    pub model: DrawInput,
}

impl RenderObject {
    pub fn render_this<'a>(&self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(self.pipeline);
        render_pass.set_bind_group(0, self.bind_group, &[]);

        match &self.model {
             DrawInput::NonIndexed {verticies, instances} => {
                render_pass.draw(verticies.clone(), instances.clone());
            }
            DrawInput::Model{ my_model, instances} => {
                render_pass.set_vertex_buffer(0, my_model.vertex_buffer.slice(..));
                render_pass.set_index_buffer(my_model.index_buffer.slice(..), my_model.index_format.clone());
                render_pass.draw_indexed(0..my_model.num_indices, 0, instances.clone());
            }
        }

    }
}