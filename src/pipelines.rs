use crate::State;
use std::sync::Arc;

pub struct ShaderPipeline {
    pub module: Arc<wgpu::ShaderModule>,
    pub pipeline: Arc<wgpu::RenderPipeline>,
    pub layout: Arc<wgpu::PipelineLayout>,
}

pub struct VertexDesc<'a> {
    pub module: &'a wgpu::ShaderModule,
    pub entry_point: &'a str,
    pub buffers: Vec<wgpu::VertexBufferLayout<'a>>
}

pub enum FragmentDesc<'a> {
    None,
    Some {
        module: &'a wgpu::ShaderModule,
        entry_point: &'a str,
        targets: Vec<wgpu::ColorTargetState>
    }
}

impl<'a> FragmentDesc<'a> {

    fn unpack(self) -> Option<(&'a wgpu::ShaderModule, &'a str, Option<Vec<wgpu::ColorTargetState>>)> {
        match self {
            FragmentDesc::None => None,
            FragmentDesc::Some { module, entry_point, targets } => {
                Some((module, entry_point, Some(targets)))
            }
        }
    }
}

/// While `wgpu::RenderPipelineDescriptor` is already very simple\
/// it tends to be very verbose in code. The goal is create an "organizer" for\
/// containing the descriptors of a pipeline. The way I use this is I create\
/// a .rs for every .wgsl/.glsl/.spirv file and just make a struct with this trait in it.
pub trait Pipeline {

    fn shader_src<'a>() -> wgpu::ShaderSource<'a> {
        unimplemented!()
    }

    fn vertex_state<'a>(module: &'a wgpu::ShaderModule) -> VertexDesc<'a> {
        VertexDesc {
            module,
            entry_point: "main",
            buffers: vec![]
        }
    }

    fn fragment_desc<'a>(module: &'a wgpu::ShaderModule) ->FragmentDesc<'a> {
        FragmentDesc::Some {
            module,
            entry_point: "main",
            targets: vec![
                wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::R8Unorm,
                    blend: None,
                    write_mask: wgpu::ColorWrites::all()
                }
            ]
        }
    }

    fn pipeline_desc<'a>(layout: &'a wgpu::PipelineLayout, fragment: Option<wgpu::FragmentState<'a>>, vertex: wgpu::VertexState<'a>) -> wgpu::RenderPipelineDescriptor<'a> {
        wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(layout),
            vertex,
            primitive: wgpu::PrimitiveState{
                topology: Default::default(),
                strip_index_format: None,
                front_face: Default::default(),
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: Default::default(),
                conservative: false
            },
            depth_stencil: None,
            multisample: Default::default(),
            fragment,
            multiview: None
        }
    }
}

/// Instantiate a rendering pipeline from a defined `Pipeline` trait.
pub fn make_pipline<'a, T: Pipeline>(device: &wgpu::Device, bind_group_layouts: &[&'a wgpu::BindGroupLayout], push_constant_ranges: &'a [wgpu::PushConstantRange]) -> ShaderPipeline {

    let module = Arc::new(device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: None,
        source: T::shader_src()
    }));

    let layout = Arc::new(device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: None,
        bind_group_layouts,
        push_constant_ranges
    }));

    let mut vstate = T::vertex_state(module.as_ref());
    let fstate = T::fragment_desc(module.as_ref()).unpack();

    let mut fstate_targets: Option<Vec<wgpu::ColorTargetState>> = None;
    let mut targets_unwraped: Vec<wgpu::ColorTargetState>;

    let pipeline = Arc::new(device.create_render_pipeline(&T::pipeline_desc(layout.as_ref(), match fstate {
        None => None,
        Some (( module, entry_point, mut targets)) => {

            std::mem::swap(&mut targets, &mut fstate_targets);
            targets_unwraped = fstate_targets.expect("[wgpu_quick] Error swapping Color Target reference!");
            Some(wgpu::FragmentState {
                module,
                entry_point,
                targets: targets_unwraped.as_slice()
            })
        }
    }, wgpu::VertexState {
        module: vstate.module,
        entry_point: vstate.entry_point,
        buffers: vstate.buffers.as_slice()
    })));

    ShaderPipeline {
        module,
        pipeline,
        layout
    }

}

