struct Vertex {
    pos: [f32; 3]
}

impl wgpu_quick::VertexType for Vertex {
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
