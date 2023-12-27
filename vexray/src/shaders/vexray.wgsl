@group(0) @binding(0) var color_buffer: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let pos: vec2<i32> = vec2<i32>(i32(id.x), i32(id.y));

    let g: f32 = f32(pos.y) / 1080.0;
    let b: f32 = f32(pos.x) / 1920.0;

    var color: vec3<f32> = vec3<f32>(0.0, g, b);

    textureStore(color_buffer, pos, vec4<f32>(color, 1.0));
}