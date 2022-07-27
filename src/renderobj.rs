use crate::rect::WorldPoint;
use wgpu::Buffer;
use crate::State;
use crate::model::Model;
use std::ops::Range;
use std::sync::Arc;

pub enum DrawInput {
    Model{my_model: Model, instances: Range<u32> },
    NonIndexed{ verticies: Range<u32>, instances: Range<u32>},
}

/// Data for a renderable object.
pub struct RenderObject {
    /// The pipeline to render with.
    pub pipeline: Arc<wgpu::RenderPipeline>,
    /// The bind groups in order of compatible `BindGroupLayouts` in the `PipelineLayout`.
    pub bind_groups: Vec<Arc<wgpu::BindGroup>>,
    /// The model buffers or indexing data for the GPU
    pub model: DrawInput,
}

impl RenderObject {
    pub fn render_this<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(self.pipeline.as_ref());
        for i in 0..self.bind_groups.len() {
            render_pass.set_bind_group(i as u32, self.bind_groups[i].as_ref(), &[]);
        }

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


    pub fn render_with_draw_args<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, vertices: Range<u32>, instances: Range<u32>) {
        // attempting to draw no instances results in (exit code: 0xc000041d)
        if instances.start != instances.end {
            render_pass.set_pipeline(self.pipeline.as_ref());
            for i in 0..self.bind_groups.len() {
                render_pass.set_bind_group(i as u32, self.bind_groups[i].as_ref(), &[]);
            }

            render_pass.draw(vertices.clone(), instances.clone());
        }else{
            println!("skipping draw, no instances");
        }
    }
}