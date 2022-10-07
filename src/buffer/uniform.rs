use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut, Range};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// A smart pointer that synchronizes a uniform buffer.
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

/// Creates a uniform that can be buffered to in sized chunks, from any index in the data to any index in the uniform.
pub struct UniformChunk<T: bytemuck::Zeroable + bytemuck::Pod> {
    buffer: Arc<wgpu::Buffer>,
    data: Vec<T>,
}

impl<T: bytemuck::Zeroable + bytemuck::Pod> Deref for UniformChunk<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}


impl<T: bytemuck::Zeroable + bytemuck::Pod> DerefMut for UniformChunk<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: bytemuck::Zeroable + bytemuck::Pod> UniformChunk<T> {
    pub fn new(device: &wgpu::Device, data: Vec<T>) -> Self {
        let buffer = Arc::new(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: None,
            contents: bytemuck::cast_slice(data.as_slice()),
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

    pub fn sync(&self, queue: &wgpu::Queue, data_index: usize, uniform_index: usize) {
        queue.write_buffer(self.buffer.as_ref(), (uniform_index * std::mem::size_of::<T>()) as u64, bytemuck::cast_slice(&[self.data.as_slice()[data_index]]))
    }

    pub fn sync_range(&self, queue: &wgpu::Queue, data_range: Range<usize>, uniform_range: Range<usize>) {
        queue.write_buffer(self.buffer.as_ref(), (uniform_range.start * std::mem::size_of::<T>()) as u64, bytemuck::cast_slice(&self.data.as_slice()[data_range]))
    }

    pub fn sync_all(&self, queue: &wgpu::Queue) {
        queue.write_buffer(self.buffer.as_ref(), 0, bytemuck::cast_slice(self.data.as_slice()))
    }

}


/// Creates a uniform in GPU memory but does not store a copy on the CPU side.\
/// Basically uses wgpu uniforms as they're made, not much abstraction besides the type argument.
pub struct UniformRemote<T: bytemuck::Zeroable + bytemuck::Pod> {
    buffer: Arc<wgpu::Buffer>,
    size: usize,
    datatype: PhantomData<T>
}

impl<T: bytemuck::Zeroable + bytemuck::Pod> UniformRemote<T> {
    pub fn new(device: &wgpu::Device, data: &[T]) -> Self {
        let buffer = Arc::new(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: None,
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        }));
        Self {
            buffer,
            size: std::mem::size_of::<T>() * data.len(),
            datatype: Default::default()
        }
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        self.buffer.as_ref()
    }

    /// Writes to the buffer, the offset is sized as if indexing `&[T]`\
    /// `queue.write_buffer()` Fails here if the size of `data` overruns the size of the buffer.
    pub fn write(&self, queue: &wgpu::Queue, index_offset: usize, data: &[T]) {
        queue.write_buffer(self.buffer.as_ref(), (index_offset * std::mem::size_of::<T>()) as u64, bytemuck::cast_slice(data))
    }
}

