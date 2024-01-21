// CONSTANTS_START
const INF_F32 = 0x1p+127f;
const EPSILON = 0x1p-149f;
const ERR_COLOR = vec3f(1.0, 0.0, 1.0);
// CONSTANTS_END

// UTILS_START
fn vec3f_len_squared(v: vec3f) -> f32 {
    return v.x * v.x + v.y * v.y + v.z * v.z;
}

fn vec3f_near_zero(v: vec3f) -> bool {
    return v.x < EPSILON && v.y < EPSILON && v.z < EPSILON;
}

fn vec3f_reflect(v: vec3f, n: vec3f) -> vec3f {
    return v - 2.0 * dot(v, n) * n;
}

fn vec3f_refract(uv: vec3f, n: vec3f, etai_over_etat: f32) -> vec3f {
    let cos_theta = min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_para = -sqrt(abs(1.0 - vec3f_len_squared(r_out_perp))) * n;

    return r_out_perp + r_out_para;
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
    let p = random_vec3f_range(-1.0, 1.0);
    if vec3f_len_squared(p) >= 1.0 {
        return normalize(p);
    } else {
        return p;
    }
}

fn random_in_unit_disk() -> vec3f {
    let p = vec3f(random_float_range(-1.0, 1.0), random_float_range(-1.0, 1.0), 0.0);
    if vec3f_len_squared(p) >= 1.0 {
        return normalize(p);
    } else {
        return p;
    }
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
    samples: u32,
    bounces: u32
}
// IMAGE_END

// CAMERA_START
struct Camera {
    center: vec3f,
    dof_angle: f32,
    dof_disk_u: vec3f,
    dof_disk_v: vec3f
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

// EXECUTION_CONTEXT_START
struct ExecutionContext {
    tile_position: vec2u
}
// EXECUTION_CONTEXT_END

// HITRECORD_START
struct HitRecord {
    t: f32,
    point: vec3f,
    normal: vec3f,
    mat_type: u32,
    mat_index: u32,
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

// MATERIAL_START
fn scatter(ray: Ray, hit: HitRecord, attenuation: ptr<function, vec3f>, scattered: ptr<function, Ray>) -> bool {
    switch hit.mat_type {
        case 1u: {
            return scatter_diffuse(ray, hit, attenuation, scattered);
        }
        case 2u: {
            return scatter_metal(ray, hit, attenuation, scattered);
        }
        case 3u: {
            return scatter_dielectric(ray, hit, attenuation, scattered);
        }
        default: {
            return false;
        }
    }
}

struct DiffuseMat {
    albedo: vec3f
}

fn scatter_diffuse(ray: Ray, hit: HitRecord, attenuation: ptr<function, vec3f>, scattered: ptr<function, Ray>) -> bool {
    let material = diffuse_mats[hit.mat_index];
    var scatter_direction = hit.normal + random_unit_vector();

    if vec3f_near_zero(scatter_direction) {
        scatter_direction = hit.normal;
    }

    (*scattered) = Ray(hit.point, scatter_direction);
    (*attenuation) = material.albedo;

    return true;
}

struct MetalMat {
    albedo: vec3f,
    roughness: f32
}

fn scatter_metal(ray: Ray, hit: HitRecord, attenuation: ptr<function, vec3f>, scattered: ptr<function, Ray>) -> bool {
    let material = metal_mats[hit.mat_index];
    let reflected = vec3f_reflect(normalize(ray.direction), hit.normal);

    (*scattered) = Ray(hit.point, reflected + material.roughness * random_unit_vector());
    (*attenuation) = material.albedo;

    return true;
}

struct DielectricMat {
    ior: f32
}

fn scatter_dielectric(ray: Ray, hit: HitRecord, attenuation: ptr<function, vec3f>, scattered: ptr<function, Ray>) -> bool {
    let material = dielectric_mats[hit.mat_index];

    (*attenuation) = vec3f(1.0);

    let refraction_ratio = select(material.ior, 1.0 / material.ior ,hit.front_face);

    let unit_direction = normalize(ray.direction);

    let cos_theta = min(dot(-unit_direction, hit.normal), 1.0);
    let sin_theta = sqrt(1.0 - cos_theta * cos_theta);

    var direction: vec3f;

    let cannot_refract = refraction_ratio * sin_theta > 1.0;

    if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_float() {
        direction = vec3f_reflect(unit_direction, hit.normal);
    } else {
        direction = vec3f_refract(unit_direction, hit.normal, refraction_ratio);
    }

    (*scattered) = Ray(hit.point, direction);

    return true;
}

fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
    var r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * pow((1.0 - cosine), 5.0);
}
// MATERIAL_END

// hit interface
// fn hit(shape: Shape, ray: Ray, ray_limits: Interval, hit: ptr<function, HitRecord>) -> bool {}

// SPHERE_START
struct Sphere {
    center: vec3f,
    radius: f32,
    mid: vec4u
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

    (*hit).mat_type = sphere.mid.x;
    (*hit).mat_index = sphere.mid.y;

    hit_set_face_normal(hit, ray, out_normal);

    return true;
}
fn hit_spheres(ray: Ray, ray_limits: Interval, hit: ptr<function, HitRecord>) -> bool {
    var temp_hit = HitRecord();
    var hit_anything = false;
    var closest_so_far = ray_limits.max;

    // arrayLength returns a u32, so we make i also u32 to make logical operation happy
    for (var i = 0u; i < arrayLength(&spheres); i++) {
        let sphere = spheres[i];

        if hit_sphere(sphere, ray, Interval(ray_limits.min, closest_so_far), &temp_hit) {
            hit_anything = true;

            closest_so_far = temp_hit.t;

            *hit = temp_hit;
        }
    }

    return hit_anything;
}
// SPHERE_END

// RENDERER_START
fn render_ray(ray: Ray) -> vec3f {
    var current_ray_origin = ray.origin;
    var current_ray_direction = ray.direction;

    // start with background color
    let unit_dir = normalize(ray.direction);
    let alpha = 0.5 * (unit_dir.y + 1.0);

    var accumulated_color = (1.0 - alpha) * vec3f(1.0) + alpha * vec3f(0.3, 0.6, 1.0);

    var bounce = 0u;

    // try world hits
    for (bounce = 0u; bounce < config.image.bounces; bounce++) {
        var hit = HitRecord();
        let ray = Ray(current_ray_origin, current_ray_direction);

        if hit_spheres(ray, Interval(0.001, INF_F32), &hit) {
            var scatter = Ray();
            var attenuation = vec3f();

            if scatter(ray, hit, &attenuation, &scatter) {
                current_ray_origin = scatter.origin;
                current_ray_direction = scatter.direction;

                accumulated_color *= attenuation;
            } else { // else should never happen
                accumulated_color = ERR_COLOR;
                break;
            }
        } else {
            break;
        }
    }

    // max bounce condition
    if bounce >= config.image.bounces {
        accumulated_color = vec3f(0.0, 0.0, 0.0);
    }

    return accumulated_color;
}

fn render(pixel_position: vec2i) -> vec4f {
    let pixel_center = config.pixel_zero_loc 
        + (f32(pixel_position.x) * config.viewport.delta_u) 
        + (f32(pixel_position.y) * config.viewport.delta_v);

    let pixel_sample = pixel_center + sample_square();
    
    let ray_origin = select(dof_disk_sample(), config.camera.center, config.camera.dof_angle <= 0.0);
    let ray_direction = pixel_sample - ray_origin;

    let ray = Ray(ray_origin, ray_direction);

    let pixel_color = render_ray(ray);

    return vec4f(pixel_color, 1.0);
}

fn sample_square() -> vec3f {
    return ((-0.5 + random_float()) * config.viewport.delta_u) + ((-0.5 + random_float()) * config.viewport.delta_v);
}

fn dof_disk_sample() -> vec3f {
    let p = random_in_unit_disk();
    return config.camera.center + (p.x * config.camera.dof_disk_u) + (p.y * config.camera.dof_disk_v);
}
// RENDERER_END

// BINDINGS_START
// System Bindings
@group(0) @binding(0) var result: texture_storage_2d<rgba8unorm, write>; // output image

// User Bindings
// - Config bindings
@group(1) @binding(0) var<uniform> config: Config; // render config
// - Scene bindings
@group(1) @binding(1) var<storage, read> spheres: array<Sphere>; // move to different group
// - Material Bindings
@group(1) @binding(2) var<storage, read> diffuse_mats: array<DiffuseMat>;
@group(1) @binding(3) var<storage, read> metal_mats: array<MetalMat>;
@group(1) @binding(4) var<storage, read> dielectric_mats: array<DielectricMat>;

// Execution Context Bindings
@group(2) @binding(0) var<uniform> execution_context: ExecutionContext; // current execution context
// BINDINGS_END

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) id: vec3u) {
    random_init(id);

    let pixel_position = vec2i(i32(id.x), i32(id.y));

    var pixel_color = vec4f();

    for (var i = 0u; i < config.image.samples; i++) {
        pixel_color += render(pixel_position);
    }

    pixel_color /= f32(config.image.samples);

    // sqrt applies gamma 2 transformation
    textureStore(result, pixel_position, sqrt(pixel_color)); // final output
}