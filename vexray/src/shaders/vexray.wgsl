// IMAGE_START
struct Image {
    width: u32,
    height: u32,
}
// IMAGE_END

// CAMERA_START
struct Camera {
    center: vec3f,
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

fn ray_color(ray: Ray) -> vec3f {
    let sphere_center = vec3f(0.0, 0.0, -1.0);

    let t = hit_sphere(sphere_center, 0.5, ray);

    if t > 0.0 {
        let normal = normalize(ray_at(ray, t) - sphere_center);
        return 0.5 * vec3f(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }

    let unit_dir = normalize(ray.direction);
    let alpha = 0.5 * (unit_dir.y + 1.0);
    return (1.0 - alpha) * vec3f(1.0, 1.0, 1.0) + alpha * vec3f(0.3, 0.6, 1.0); // lerp
}
// RAY_END

// SPHERE_START
/// solves the sphere ray intersection equation, which is a quadratic equation
fn hit_sphere(center: vec3f, radius: f32, ray: Ray) -> f32 {
    let origin_to_center = ray.origin - center; // A - C

    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(origin_to_center, ray.direction);
    let c = dot(origin_to_center, origin_to_center) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - sqrt(discriminant)) / (2.0 * a); // '-' -> one solution only
    }
}
// SPHERE_END

// BINDINGS_START
@group(0) @binding(0) var result: texture_storage_2d<rgba8unorm, write>; // output image
@group(0) @binding(1) var<uniform> config: Config; // render config
// BINDINGS_END

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) id: vec3u) {
    let pixel_position = vec2i(i32(id.x), i32(id.y));

    let pixel_center = config.pixel_zero_loc 
        + (f32(pixel_position.x) * config.viewport.delta_u) 
        + (f32(pixel_position.y) * config.viewport.delta_v);
    
    let ray_direction = pixel_center - config.camera.center;

    let ray = Ray(config.camera.center, ray_direction);

    var pixel_color = ray_color(ray);

    textureStore(result, pixel_position, vec4f(pixel_color, 1.0)); // final output
}