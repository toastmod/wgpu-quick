use wgpu_quick::pipelines::{Pipeline, VertexDesc, FragmentDesc};
use std::ops::Index;
use wgpu::{ShaderSource, ShaderModule, PipelineLayout, FragmentState, VertexState, RenderPipelineDescriptor};
use std::borrow::Cow;
use wgpu_quick::State;

pub struct TrianglePipe;

impl Pipeline for TrianglePipe {
    fn shader_src<'a>(state: &State) -> ShaderSource<'a> {
        ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl")))
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
            targets: vec![state.config.format.clone().into()]
        }
    }

    fn pipeline_desc<'a>(state: &State, layout: Option<&'a PipelineLayout>, fragment: Option<FragmentState<'a>>, vertex: VertexState<'a>) -> RenderPipelineDescriptor<'a> {
        wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex,
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment,
            multiview: None
        }
    }
}