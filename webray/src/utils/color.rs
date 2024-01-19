use rand::{rngs::ThreadRng, Rng};

pub fn random_color(rng: &mut ThreadRng) -> glam::Vec3 {
    let r: f32 = rng.gen();
    let g: f32 = rng.gen();
    let b: f32 = rng.gen();

    return glam::vec3(r, g, b);
}

pub fn random_color_range(rng: &mut ThreadRng, min: f32, max: f32) -> glam::Vec3 {
    let r: f32 = rng.gen_range(min..max);
    let g: f32 = rng.gen_range(min..max);
    let b: f32 = rng.gen_range(min..max);

    return glam::vec3(r, g, b);
}