struct VOut {
    @location(0) uv: vec2<f32>,
    @builtin(position) pos: vec4<f32>
}

@vertex
fn vs_main(
  @builtin(vertex_index) in_vindx: u32
) -> VOut {
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, 1.0),
    );

    let p = vec4<f32>(pos[in_vindx], 0.0, 1.0);
    let uv = vec2<f32>((p.x+1.0)/2.0, 1.0-((p.y+1.0)/2.0));
    var res: VOut;
    res.pos = p;
    res.uv = uv;

    return res;

}

@group(0) @binding(0)
var tview: texture_2d<f32>;

@group(0) @binding(1)
var tsamp: sampler;

@fragment
fn fs_main(vin: VOut) -> @location(0) vec4<f32> {
    return textureSample(tview, tsamp, vin.uv);
}