#[derive(Debug, encase::ShaderType)]
pub struct KSphere {
    pub center: glam::Vec3,
    pub radius: f32,
    pub mat_type: u32,
    pub mat_index: u32
}