// CONSTANTS_START
const INF_F32 = 0x1p+127f;
// CONSTANTS_END

// UTILS_START
fn vec3f_len_squared(v: vec3f) -> f32 {
    return v.x * v.x + v.y * v.y + v.z * v.z;
}

struct Interval {
    min: f32,
    max: f32
}

fn interval_contains(interval: Interval, x: f32) -> bool {
    return interval.min <= x && x <= interval.max;
}

fn interval_surrounds(interval: Interval, x: f32) -> bool {
    return interval.min < x && x < interval.max;
}
// UTILS_END

// RNG_START - Pcg32 modified
// https://github.com/grigoryoskin/vulkan-compute-ray-tracing/blob/master/resources/shaders/source/include/random.glsl
struct Rng {
    state: u32
}

var<private> rng: Rng;

fn step_rng(state: u32) -> u32 {
    return state * 747796405u + 1u;
}

fn pcg32(_rng: ptr<private, Rng>) -> u32 {
    let old_state = (*_rng).state;

    let new_state = step_rng(old_state);

    (*_rng).state = new_state;

    let w1 = ((new_state >> ((new_state >> 28u) + 4u)) ^ new_state) * 277803737u;
    let w2 = (w1 >> 22u) ^ w1;

    return w2;
}

fn random_init(seed: vec3u) {
    rng = Rng((seed.x * seed.y) + 1337u);
}

fn random_float() -> f32 {
    return f32(pcg32(&rng)) / 4294967295.0;
}

fn random_float_range(min: f32, max: f32) -> f32 {
    return min + (max - min) * random_float();
}

fn random_vec3f() -> vec3f {
    return vec3f(random_float(), random_float(), random_float());
}

fn random_vec3f_range(min: f32, max: f32) -> vec3f {
    return vec3f(random_float_range(min, max), random_float_range(min, max), random_float_range(min, max));
}

fn random_in_unit_sphere() -> vec3f {
    loop {
        let p = random_vec3f_range(-1.0, 1.0);
        if vec3f_len_squared(p) < 1.0 {
            return p;
        }
    }
    return vec3f(); // never reach here
}

fn random_unit_vector() -> vec3f {
    return normalize(random_in_unit_sphere());
}

fn random_on_hemisphere(normal: vec3f) -> vec3f {
    let unit = random_unit_vector();

    if dot(unit, normal) > 0.0 { // same side
        return unit;
    } else {
        return -unit;
    }
}
// RNG_END

// IMAGE_START
struct Image {
    width: u32,
    height: u32,
}
// IMAGE_END

// CAMERA_START
struct Camera {
    center: vec3f,
    focal_length: f32,
    samples: u32,
    bounces: u32
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

// HITRECORD_START
struct HitRecord {
    point: vec3f,
    normal: vec3f,
    t: f32,
    front_face: bool
}

/// Uses dot product to figure out which side the ray is
/// out_normal needs to be a unit vector
fn hit_set_face_normal(hit: ptr<function, HitRecord>, ray: Ray, out_normal: vec3f) {
    let front_face = dot(ray.direction, out_normal) < 0.0;
    let normal = select(-out_normal, out_normal, front_face);

    (*hit).front_face = front_face;
    (*hit).normal = normal;
}
// HITRECORD_END

// RAY_START
struct Ray {
    origin: vec3f,
    direction: vec3f
}

fn ray_at(ray: Ray, t: f32) -> vec3f {
    return ray.origin + t * ray.direction;
}
// RAY_END

// hit interface
// fn hit(shape: Shape, ray: Ray, ray_limits: Interval, hit: ptr<function, HitRecord>) -> bool {}

// SPHERE_START
struct Sphere {
    center: vec3f,
    radius: f32
}

/// solves the sphere ray intersection equation, which is a quadratic equation
fn hit_sphere(sphere: Sphere, ray: Ray, ray_limits: Interval, hit: ptr<function, HitRecord>) -> bool {
    let origin_to_center = ray.origin - sphere.center; // A - C

    let a = vec3f_len_squared(ray.direction);
    let half_b = dot(origin_to_center, ray.direction);
    let c = vec3f_len_squared(origin_to_center) - sphere.radius * sphere.radius;

    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return false;
    }

    let sqrtd = sqrt(discriminant);

    var root = (-half_b - sqrtd) / a;
    if !interval_surrounds(ray_limits, root) {
        root = (-half_b + sqrtd) / a;
        if !interval_surrounds(ray_limits, root) {
            return false;
        }
    }

    let point = ray_at(ray, root);
    let out_normal = (point - sphere.center) / sphere.radius; // this will be unit length

    (*hit).t = root;
    (*hit).point = point;
    hit_set_face_normal(hit, ray, out_normal);

    return true;
}
// SPHERE_END

// WORLD_START
fn hit_world(ray: Ray, ray_limits: Interval, hit: ptr<function, HitRecord>) -> bool {
    var temp_hit = HitRecord();
    var hit_anything = false;
    var closest_so_far = ray_limits.max;

    // arrayLength returns a u32, so we make i also u32 to make logical operation happy
    for (var i = 0u; i < arrayLength(&world); i++) {
        let sphere = world[i];

        if hit_sphere(sphere, ray, Interval(ray_limits.min, closest_so_far), &temp_hit) {
            hit_anything = true;
            closest_so_far = temp_hit.t;
            *hit = temp_hit;
        }
    }

    return hit_anything;
}
// WORLD_END

// RENDERER_START
fn render_ray(ray: Ray) -> vec3f {
    var hit = HitRecord();

    if hit_world(ray, Interval(0.0, INF_F32), &hit) {
        return 0.5 * (hit.normal + vec3f(1.0));
    }

    let unit_dir = normalize(ray.direction);
    let alpha = 0.5 * (unit_dir.y + 1.0);

    return (1.0 - alpha) * vec3f(1.0, 1.0, 1.0) + alpha * vec3f(0.3, 0.6, 1.0); // lerp
}

fn render_ray_v2(ray: Ray) -> vec3f {
    var current_ray_origin = ray.origin;
    var current_ray_direction = ray.direction;

    // start with background color
    let unit_dir = normalize(ray.direction);
    let alpha = 0.5 * (unit_dir.y + 1.0);

    var accumulated_color = (1.0 - alpha) * vec3f(1.0, 1.0, 1.0) + alpha * vec3f(0.3, 0.6, 1.0);

    var bounce = 0u;

    // try world hits
    for (bounce = 0u; bounce < config.camera.bounces; bounce++) {
        var hit = HitRecord();

        if hit_world(Ray(current_ray_origin, current_ray_direction), Interval(0.001, INF_F32), &hit) {
            let direction = hit.normal + random_unit_vector();

            current_ray_origin = hit.point;
            current_ray_direction = direction;

            accumulated_color *= 0.5;
        } else {
            break;
        }
    }

    // max bounce condition
    if bounce >= config.camera.bounces {
        accumulated_color = vec3f();
    }

    return accumulated_color;
}

fn render(pixel_position: vec2i) -> vec4f {
    let pixel_center = config.pixel_zero_loc 
        + (f32(pixel_position.x) * config.viewport.delta_u) 
        + (f32(pixel_position.y) * config.viewport.delta_v);
    
    let sample_square = ((-0.5 + random_float()) * config.viewport.delta_u) + ((-0.5 + random_float()) * config.viewport.delta_v);

    let pixel_sample = pixel_center + sample_square;
    
    let ray_direction = pixel_sample - config.camera.center;

    let ray = Ray(config.camera.center, ray_direction);

    let pixel_color = render_ray_v2(ray);

    return vec4f(pixel_color, 1.0);
}
// RENDERER_END

// BINDINGS_START
@group(0) @binding(0) var result: texture_storage_2d<rgba8unorm, write>; // output image
@group(0) @binding(1) var<uniform> config: Config; // render config
@group(0) @binding(2) var<storage, read> world: array<Sphere>;
// BINDINGS_END

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) id: vec3u) {
    random_init(id);

    let pixel_position = vec2i(i32(id.x), i32(id.y));

    var pixel_color = vec4f();

    for (var i = 0u; i < config.camera.samples; i++) {
        pixel_color += render(pixel_position);
    }

    pixel_color /= f32(config.camera.samples);

    textureStore(result, pixel_position, pixel_color); // final output
}