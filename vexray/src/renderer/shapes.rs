/// Had to make size of KSphere a multiple of 2
/// either ensure encase does tight packing or somehow specify a offset in wgpu
#[derive(Debug, encase::ShaderType)]
pub struct KSphere {
    pub center: glam::Vec3,
    pub radius: f32,
    pub mid: glam::UVec4
}