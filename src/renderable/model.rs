use std::{ops::{Range, Index}, sync::Arc};

use crate::{buffer::vertex::{VertexType, VertexBuffer}, State, init};
use super::{Indices, Renderable};

use wgpu::util::DeviceExt;


/// Data for a 3D model, and it's format.
pub struct Model<V: VertexType> {
    pub vertex_buffer: Option<VertexBuffer<V>>,
    pub indexing: Indices
}

impl<V: VertexType> Model<V> {
    /// Use raw obj text file data
    pub fn from_raw_obj(state: &State, data: &[u8]) -> Arc<Self> {

        unimplemented!();

        let obj = obj::raw::parse_obj(data).expect("Model data could not be parsed!");

        Arc::new(Self {
            vertex_buffer: todo!(),
            indexing: todo!()
        })
    }

    pub fn from_vertices(state: &State, vertices: &[V], indices: Indices) -> Arc<Self> {
        Arc::new(Self {
            vertex_buffer: Some(VertexBuffer::<V>::new(&state.device, vertices)),
            indexing: indices,
        })
    }

    pub fn empty(indices: Indices) -> Arc<Self> {
        Arc::new(Self {
            vertex_buffer: None, 
            indexing: indices,
        })
    }
}

unsafe impl VertexType for () {
    fn attrib_layout<'a>() -> &'a[wgpu::VertexAttribute] {
        &[]
    }
}

impl<V: VertexType> Renderable for Model<V> {
    fn indices(&self) -> &Indices {
        &self.indexing
    }

    fn vertices(&self) -> Option<&wgpu::Buffer> {
        match &self.vertex_buffer {
            Some(vb) => Some(&vb.buffer),
            None => None
        }
    }

}
