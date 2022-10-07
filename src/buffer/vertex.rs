use std::sync::Arc;

use wgpu::util::DeviceExt;
use std::marker::PhantomData;

pub struct VertexAttribute {
    size: usize,
}
impl VertexAttribute {
    fn reg<T>(shader_location: usize) -> Self {
        Self {
            size: std::mem::size_of::<T>()
        }
    }
}


pub fn construct_vbo_layout(format: &[VertexAttribute]){
    unimplemented!()
}

pub trait VertexFormat {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        unimplemented!();
        // wgpu::VertexBufferLayout { array_stride: 0, step_mode: wgpu::VertexStepMode::Vertex, attributes: &[
        //     wgpu::VertexAttribute {
        //         format: VertexFormat::Uint8x2,
        //         offset: 0,
        //         shader_location: 0
        //     }
        // ] }
    } 
}

/// A buffer that stores a certain vertex format.
pub struct VertexBuffer<V: bytemuck::Zeroable + bytemuck::Pod + VertexFormat> {
    buffer: Arc<wgpu::Buffer>,
    ph: PhantomData<V>
}

impl<V: bytemuck::Zeroable + bytemuck::Pod + VertexFormat> VertexBuffer<V> {
    pub fn new(device: &wgpu::Device, data: &[V]) -> Self {
        let buffer = Arc::new(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: None,
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        }));
        Self {
            buffer,
            ph: Default::default()
        }
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        self.buffer.as_ref()
    }

    pub fn sync(&self, queue: &wgpu::Queue) {
        unimplemented!()
        // queue.write_buffer(self.buffer.as_ref(), 0, bytemuck::cast_slice(&[self.data]))
    }
}