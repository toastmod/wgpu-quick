use std::marker::PhantomData;

use wgpu::util::DeviceExt;
#[macro_use]
pub use wgpu::vertex_attr_array;
pub unsafe trait VertexType: Sized + crate::bytemuck::Pod + crate::bytemuck::Zeroable {
    /// The individual formats of each attribute.
    fn attrib_layout<'a>() -> &'a[wgpu::VertexAttribute];

    /// The description of the Vertex layout for a buffer.
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::attrib_layout()
        }
    }   
}


/// A Vertex Buffer reference.
pub struct VertexBuffer<V: VertexType + crate::bytemuck::Pod + crate::bytemuck::Zeroable> {
    pub buffer: wgpu::Buffer,
    _vertex_type: PhantomData<V>
}

impl<V: VertexType + crate::bytemuck::Pod + crate::bytemuck::Zeroable> VertexBuffer<V> {
    pub fn new(device: &wgpu::Device, data: &[V]) -> Self {
        println!("Creating vertex buffer");
        Self { 
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: None,
                contents: crate::bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            _vertex_type: PhantomData,   
        }
    }
    pub fn set_to_render<'a>(&'a self, rpass: &'a mut wgpu::RenderPass<'a>, location: u32) {
        rpass.set_vertex_buffer(location, self.buffer.slice(..));
    }
} 

