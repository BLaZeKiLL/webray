#[derive(Debug, encase::ShaderType)]
pub struct Sphere {
    center: glam::Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: glam::Vec3, radius: f32) -> Self {
        return Self { center, radius };
    }
}