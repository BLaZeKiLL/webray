use super::shapes::Sphere;

#[derive(Debug, encase::ShaderType)]
pub struct World {
    #[size(runtime)]
    spheres: Vec<Sphere>
}

impl World {
    pub fn new() -> Self {
        return World { spheres: Vec::new() };
    }

    pub fn add_sphere(&mut self, center: glam::Vec3, radius: f32) {
        self.spheres.push(Sphere::new(center, radius));
    }

    pub fn sphere_objects_as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(&self.spheres).unwrap();
        return Ok(buffer.into_inner());
    }
}