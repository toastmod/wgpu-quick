#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: [f32; 2]
}

unsafe impl wgpu_quick::bytemuck::Pod for Vertex {}
unsafe impl wgpu_quick::bytemuck::Zeroable for Vertex {}

unsafe impl wgpu_quick::buffer::vertex::VertexType for Vertex {
    fn attrib_layout<'a>() -> &'a[wgpu::VertexAttribute] {
        &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2
            }
        ]
    }

}
