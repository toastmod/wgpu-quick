use wgpu_quick::pipelines::{Pipeline, VertexDesc, FragmentDesc};
use wgpu_quick::State;
use wgpu::{ShaderSource, ShaderModule, PipelineLayout, FragmentState, VertexState, RenderPipelineDescriptor};
use std::borrow::Cow;

pub struct MouseTexPipeline;
impl Pipeline for MouseTexPipeline {
    fn shader_src<'a>(state: &State) -> ShaderSource<'a> {
        wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl")))
    }

    fn vertex_state<'a>(state: &State, module: &'a ShaderModule) -> VertexDesc<'a> {
        VertexDesc {
            module,
            entry_point: "vs_main",
            buffers: vec![]
        }
    }

    fn fragment_desc<'a>(state: &State, module: &'a ShaderModule) -> FragmentDesc<'a> {
        FragmentDesc::Some {
            module,
            entry_point: "fs_main",
            targets: vec![
                state.config.format.clone().into()
            ]
        }
    }

    fn pipeline_desc<'a>(state: &State, layout: Option<&'a PipelineLayout>, fragment: Option<FragmentState<'a>>, vertex: VertexState<'a>) -> RenderPipelineDescriptor<'a> {
        wgpu::RenderPipelineDescriptor {
            label: None,
            layout,
            vertex,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: None,
            multisample: Default::default(),
            fragment,
            multiview: None
        }
    }
}