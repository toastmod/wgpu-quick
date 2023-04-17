use std::ops::Range;
use std::sync::Arc;

use wgpu::RenderPipeline;
use wgpu::util::DeviceExt;
use crate::pipelines::Pipeline;

use super::State;

/// Data for a renderable object.
pub mod model;

/// Data for indexing vertices
pub enum Indices {
    IndexBuffer { index_buffer: wgpu::Buffer, index_format: wgpu::IndexFormat, num_indices: u32, instances: Range<u32> },
    Ranged { vertices: Range<u32>, instances: Range<u32>},
}

impl Indices {
    pub fn from_indices<I: crate::bytemuck::Pod + crate::bytemuck::Zeroable>(state: &State, indices: &[I], index_format: wgpu::IndexFormat, instances: Range<u32>) -> Self {
        println!("Creating index buffer");
        Indices::IndexBuffer {
            index_buffer: state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: None,
                contents: crate::bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }) , 
            index_format, 
            num_indices: indices.len() as u32, 
            instances 
        }
    }

    // pub fn from_range() -> Self {

    // }
}

pub trait Renderable {
    fn vertices(&self) -> Option<&wgpu::Buffer>;
    fn indices(&self) -> &Indices;
}
pub struct RenderObject {
    /// The pipeline to render with.
    pub pipeline: Arc<wgpu::RenderPipeline>,
    /// The bind groups in order of compatible `BindGroupLayouts` in the `PipelineLayout`.
    pub bind_groups: Vec<Arc<wgpu::BindGroup>>,
    /// The model buffers 
    pub model: Arc<dyn Renderable>,
}

impl RenderObject {

    pub fn new(pipeline: &Arc<RenderPipeline>, bind_groups: Vec<Arc<wgpu::BindGroup>>, model: &Arc<dyn Renderable>) -> Self {
        Self {
            pipeline: Arc::clone(pipeline),
            bind_groups,
            model: Arc::clone(model),
        }
    }

    pub fn render_this<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(self.pipeline.as_ref());
        for i in 0..self.bind_groups.len() {
            println!("Setting bindgroup {}", i);
            render_pass.set_bind_group(i as u32, self.bind_groups[i].as_ref(), &[]);
        }

        // Set vertices
        match self.model.vertices() {
            Some(vertex_buffer) => {
                println!("Setting vertex buffer");
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            },
            None => {},
        };

        // Draw indices
        match &self.model.indices(){
            Indices::IndexBuffer { index_buffer, index_format, num_indices, instances } => {
                render_pass.set_index_buffer(index_buffer.slice(..), index_format.clone());
                println!("Drawing indexed");
                render_pass.draw_indexed(0..num_indices.clone(), 0, instances.clone());
            },
            Indices::Ranged { vertices, instances } => {
                println!("Drawing ranged");
                render_pass.draw(vertices.clone(), instances.clone());
            },
        }

    }


    pub fn render_with_draw_args<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, vertices: Range<u32>, instances: Range<u32>) {
        // !! attempting to draw no instances results in (exit code: 0xc000041d)
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
