use std::num::NonZeroU32;
use std::sync::Arc;

pub struct Binder<'a> {
    pub binding: u32,
    pub visibility: wgpu::ShaderStages,
    pub ty: wgpu::BindingType,
    pub count: Option<NonZeroU32>,
    pub resources: Vec<wgpu::BindingResource<'a>>
}

impl<'a> Binder<'a> {
    pub fn construct(mut self) -> (wgpu::BindGroupLayoutEntry, Vec<wgpu::BindGroupEntry<'a>>) {
        let mut groupentries = vec![];
        while !self.resources.is_empty() {
            groupentries.push(wgpu::BindGroupEntry{
                binding: self.binding.clone(),
                resource: self.resources.remove(0)
            });
        }

        (wgpu::BindGroupLayoutEntry{
            binding: self.binding,
            visibility: self.visibility,
            ty: self.ty,
            count: self.count
        }, groupentries)
    }
}

pub struct Bindings {
    pub bind_layout: wgpu::BindGroupLayout,
    pub bind_groups: Vec<Arc<wgpu::BindGroup>>,
}

impl Bindings {
    pub fn make<'a>(device: &wgpu::Device, mut bindings: Vec<Binder<'a>>) -> Self {

        let layout_size = bindings.len();

        let num_groups = {
            if layout_size == 0 {
                0
            } else {
                bindings[0].resources.len()
            }
        };

        let mut layout_entries: Vec<wgpu::BindGroupLayoutEntry> = vec![];
        let mut bind_groups: Vec<Arc<wgpu::BindGroup>> = vec![];

        //              [binding][group]
        let mut bgents: Vec<Vec<wgpu::BindGroupEntry<'a>>> = vec![];

        // fill layout entries and unmirrored bindgroup entries
        while !bindings.is_empty() {
            let (layout_ent, bg_ent) = bindings.remove(0).construct();
            layout_entries.push(layout_ent);
            bgents.push(bg_ent);
        }

        // create layout
        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: None,
            entries: layout_entries.as_slice()
        });

        // index flipped
        for _ in 0..num_groups {
            let mut entries: Vec<wgpu::BindGroupEntry> = vec![];
            println!("layout size: {}",layout_size);
            for binding_index in 0..layout_size {
                entries.push(bgents[binding_index].remove(0));
            }
            bind_groups.push(Arc::new(device.create_bind_group(&wgpu::BindGroupDescriptor{
                label: None,
                layout: &bind_layout,
                entries: entries.as_slice()
            })))
        }

        Self {

            bind_layout,
            bind_groups
        }

    }
}
