
struct Mouse {
    pos: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

[[group(0), binding(0)]]
var<uniform> mouse: Mouse;

[[group(0), binding(1)]]
var tview: texture_2d<f32>;

[[group(0), binding(2)]]
var tsamp: sampler;

[[stage(fragment)]]
fn fs_main([[builtin(position)]] p: vec4<f32>) -> [[location(0)]] vec4<f32> {
    return textureSample(tview, tsamp, mouse.pos);
}