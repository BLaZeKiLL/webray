// IMAGE_START
struct Image {
    width: u32,
    height: u32,
}
// IMAGE_END

// CAMERA_START
struct Camera {
    position: vec3f,
    focal_length: f32
}
// CAMERA_END

// VIEWPORT_START
struct Viewport {
    width: f32,
    height: f32,
    u: vec3f,
    v: vec3f,
    delta_u: vec3f,
    delta_v: vec3f,
    upper_left: vec3f,
}
// VIEWPORT_END

// CONFIG_START
struct Config {
    image: Image,
    camera: Camera,
    viewport: Viewport,
    pixel_zero_loc: vec3f
}
// CONFIG_END

// RAY_START
struct Ray {
    origin: vec3f,
    direction: vec3f
}

fn ray_at(ray: Ray, t: f32) -> vec3f {
    return ray.origin + t * ray.direction;
}
// RAY_END

// BINDINGS_START
@group(0) @binding(0) var result: texture_storage_2d<rgba8unorm, write>; // output image
@group(0) @binding(1) var<uniform> config: Config; // render config
// BINDINGS_END

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) id: vec3u) {
    let pixel_position: vec2i = vec2i(i32(id.x), i32(id.y));

    let g: f32 = f32(pixel_position.y) / f32(config.image.height);
    let b: f32 = f32(pixel_position.x) / f32(config.image.width);

    var pixel_color: vec3f = vec3f(0.0, g, b);

    textureStore(result, pixel_position, vec4f(pixel_color, 1.0)); // final output
}