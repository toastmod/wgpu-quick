use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub struct Uniform<T: bytemuck::Zeroable + bytemuck::Pod> {
    buffer: Arc<wgpu::Buffer>,
    data: T,
}

impl<T: bytemuck::Zeroable + bytemuck::Pod> Deref for Uniform<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}


impl<T: bytemuck::Zeroable + bytemuck::Pod> DerefMut for Uniform<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: bytemuck::Zeroable + bytemuck::Pod> Uniform<T> {
    pub fn new(device: &wgpu::Device, data: T) -> Self {
        let buffer = Arc::new(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: None,
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        }));
        Self {
            buffer,
            data
        }
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        self.buffer.as_ref()
    }

    pub fn sync(&self, queue: &wgpu::Queue) {
        queue.write_buffer(self.buffer.as_ref(), 0, bytemuck::cast_slice(&[self.data]))
    }
}