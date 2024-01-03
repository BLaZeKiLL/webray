use super::{shapes::KSphere, material::{KDiffuseMat, KMetalMat}};

#[derive(Debug)]
pub struct KernelScene {
    spheres: KSpheres,
    diffuse_mats: KDiffuseMats,
    metal_mats: KMetalMats
}

impl KernelScene {
    pub fn new() -> Self {
        return KernelScene {
            spheres: KSpheres { instances: Vec::new() },
            diffuse_mats: KDiffuseMats { instances: Vec::new() },
            metal_mats: KMetalMats { instances: Vec::new() },
        };
    }

    pub fn register_diffuse_material(&mut self, mat: KDiffuseMat) -> u32 {
        self.diffuse_mats.instances.push(mat);
        return self.diffuse_mats.instances.len() as u32 - 1;
    }

    pub fn register_metal_material(&mut self, mat: KMetalMat) -> u32 {
        self.metal_mats.instances.push(mat);
        return self.metal_mats.instances.len() as u32 - 1;
    }

    pub fn register_sphere(&mut self, sphere: KSphere) -> u32 {
        self.spheres.instances.push(sphere);
        return self.spheres.instances.len() as u32 - 1;
    }

    pub fn spheres_as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        dbg!(&self.spheres);
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

#[derive(Debug, encase::ShaderType)]
struct KSpheres {
    #[size(runtime)]
    instances: Vec<KSphere>
}

#[derive(Debug, encase::ShaderType)]
struct KDiffuseMats {
    #[size(runtime)]
    instances: Vec<KDiffuseMat>
}

#[derive(Debug, encase::ShaderType)]
struct KMetalMats {
    #[size(runtime)]
    instances: Vec<KMetalMat>
}