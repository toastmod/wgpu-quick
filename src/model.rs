/// Data for a 3D model, and it's format.
pub struct Model {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_format: wgpu::IndexFormat,
    pub num_indices: u32
}

impl Model {
    /// Use raw obj text file data
    pub fn from_raw_obj(data: &[u8]) -> Self {

        unimplemented!();

        let obj = obj::raw::parse_obj(data).expect("Model data could not be parsed!");

        Self {
            vertex_buffer: todo!(),
            index_buffer: todo!(),
            index_format: todo!(),
            num_indices: todo!(),
        }
    }
}