use std::collections::HashMap;

use crate::{
    renderer::{
        config::{CameraConfig, KernelConfig, RenderConfig, TileSize},
        material::{KDielectricMat, KDiffuseMat, KMetalMat},
        scene::KernelScene,
        shapes::KSphere,
    },
    utils::color::hex_to_rgb,
};

use self::types::{WMaterialType, WObjectType, WScene};

pub mod types;

impl WScene {
    pub fn get_kernel_scene(&self) -> KernelScene {
        let mut kernel_scene = KernelScene::new();

        let mut materials: HashMap<usize, (u32, u32)> = HashMap::new();

        for mat in self.materials[..].iter() {
            match &mat.mat_type {
                // because of color have to do a borrow
                WMaterialType::Diffuse { color } => {
                    let albedo = hex_to_rgb(color).unwrap();
                    let idx = kernel_scene.register_diffuse_material(KDiffuseMat { albedo });
                    materials.insert(mat.id.try_into().unwrap(), (1, idx));
                }
                WMaterialType::Metal { color, roughness } => {
                    let albedo = hex_to_rgb(color).unwrap();
                    let idx = kernel_scene.register_metal_material(KMetalMat {
                        albedo,
                        roughness: *roughness,
                    });
                    materials.insert(mat.id.try_into().unwrap(), (2, idx));
                }
                WMaterialType::Dielectric { ior } => {
                    let idx =
                        kernel_scene.register_dielectric_material(KDielectricMat { ior: *ior });
                    materials.insert(mat.id.try_into().unwrap(), (3, idx));
                }
            }
        }

        for obj in self.objects[..].iter() {
            match obj.obj_type {
                WObjectType::Sphere { position, radius } => {
                    let mat_res = materials.get(&obj.material_id);

                    match mat_res {
                        Some(mat) => {
                            kernel_scene.register_sphere(KSphere {
                                center: position,
                                radius,
                                mid: glam::uvec4(mat.0, mat.1, 0, 0),
                            });
                        }
                        None => panic!("Material not found: {}", &obj.material_id),
                    }
                }
            }
        }

        return kernel_scene;
    }

    pub fn get_kernel_config(&self) -> KernelConfig {
        let render_config = RenderConfig {
            width: self.render_settings.width,
            height: self.render_settings.height,
            samples: self.render_settings.samples,
            bounces: self.render_settings.bounces,
            tile_size: match self.render_settings.tile_size {
                types::WTileSize::Full => TileSize::Full,
                types::WTileSize::Tile { size } => TileSize::Tile(size),
            },
        };

        let camera_config = CameraConfig {
            look_from: self.camera.look_from,
            look_at: self.camera.look_at,
            v_up: self.camera.v_up,
            v_fov: self.camera.v_fov,
            dof_angle: self.camera.dof_angle,
            dof_distance: self.camera.dof_distance,
        };

        return KernelConfig::new(&render_config, &camera_config);
    }
}
