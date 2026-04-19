struct GlyphInstance {
    @location(0) pos: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) uv_pos: vec2<f32>,
    @location(3) uv_size: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(5) bg_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) bg_color: vec4<f32>,
    @location(3) uv_size: vec2<f32>,
};

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@group(1) @binding(0)
var<uniform> screen_size: vec2<f32>;

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: GlyphInstance,
) -> VertexOutput {
    var out: VertexOutput;
    
    var pos_quad = array<vec2<f32>, 4>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
    );
    
    let p = pos_quad[vertex_index];
    let world_pos = instance.pos + p * instance.size;
    
    let clip_x = (world_pos.x / screen_size.x) * 2.0 - 1.0;
    let clip_y = 1.0 - (world_pos.y / screen_size.y) * 2.0;
    
    out.clip_position = vec4<f32>(clip_x, clip_y, 0.0, 1.0);
    out.uv = instance.uv_pos + p * instance.uv_size;
    out.color = instance.color;
    out.bg_color = instance.bg_color;
    out.uv_size = instance.uv_size;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Detect solid color mode (used for continuous line highlights)
    if (in.uv_size.x < 0.00001 && in.uv_size.y < 0.00001) {
        return in.bg_color;
    }

    let alpha = textureSample(t_diffuse, s_diffuse, in.uv).r;
    let color = mix(in.bg_color, in.color, alpha);
    let final_alpha = max(in.bg_color.a, in.color.a * alpha);
    
    return vec4<f32>(color.rgb, final_alpha);
}
