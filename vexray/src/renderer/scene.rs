use super::{shapes::KSphere, material::{KDiffuseMat, KMetalMat}};

#[derive(Debug)]
pub struct KernelScene {
    spheres: Vec<KSphere>,
    diffuse_mats: Vec<KDiffuseMat>,
    metal_mats: Vec<KMetalMat>
}

impl KernelScene {
    pub fn new() -> Self {
        return KernelScene {
            spheres: Vec::new(),
            diffuse_mats: Vec::new(),
            metal_mats: Vec::new(),
        };
    }

    pub fn register_diffuse_material(&mut self, mat: KDiffuseMat) -> u32 {
        self.diffuse_mats.push(mat);
        return self.diffuse_mats.len() as u32 - 1;
    }

    pub fn register_metal_material(&mut self, mat: KMetalMat) -> u32 {
        self.metal_mats.push(mat);
        return self.metal_mats.len() as u32 - 1;
    }

    pub fn register_sphere(&mut self, sphere: KSphere) -> u32 {
        self.spheres.push(sphere);
        return self.spheres.len() as u32 - 1;
    }

    pub fn spheres_as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(&self.spheres).unwrap();
        return Ok(buffer.into_inner());
    }

    pub fn diffuse_mats_as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(&self.diffuse_mats).unwrap();
        return Ok(buffer.into_inner());
    }

    pub fn metal_mats_as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(&self.metal_mats).unwrap();
        return Ok(buffer.into_inner());
    }
}