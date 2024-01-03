#[derive(Debug, encase::ShaderType)]
pub struct KDiffuseMat {
    pub albedo: glam::Vec3
}

#[derive(Debug, encase::ShaderType)]
pub struct KMetalMat {
    pub albedo: glam::Vec3,
    pub roughness: f32
}