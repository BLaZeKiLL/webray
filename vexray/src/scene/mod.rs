use std::collections::HashMap;

use crate::renderer::{
    material::{KDiffuseMat, KMetalMat},
    scene::KernelScene, shapes::KSphere,
};

use self::{material::Material, shape::Shape};

pub mod material;
pub mod shape;

pub struct Scene {
    shapes: Vec<Shape>,
    materials: Vec<Material>,
}

impl Scene {
    pub fn new() -> Self {
        return Scene {
            shapes: Vec::new(),
            materials: Vec::new(),
        };
    }

    pub fn register_material(&mut self, material: Material) -> usize {
        self.materials.push(material);
        return self.materials.len() - 1;
    }

    pub fn register_shape(&mut self, shape: Shape) -> usize {
        self.shapes.push(shape);
        return self.shapes.len() - 1;
    }
}

impl From<Scene> for KernelScene {
    fn from(scene: Scene) -> Self {
        let mut kernel_scene = KernelScene::new();

        let mut materials: HashMap<usize, (u32, u32)> = HashMap::new();

        for (i, mat) in scene.materials.into_iter().enumerate() {
            match mat {
                Material::Diffuse(albedo) => {
                    let idx = kernel_scene.register_diffuse_material(KDiffuseMat { albedo });
                    materials.insert(i, (1, idx));
                }
                Material::Metal(albedo, roughness) => {
                    let idx = kernel_scene.register_metal_material(KMetalMat { albedo, roughness });
                    materials.insert(i, (2, idx));
                }
            }
        }

        for shape in scene.shapes {
            match shape {
                Shape::Sphere(center, radius, mat_id) => {
                    let mat = materials.get(&mat_id).unwrap();

                    kernel_scene.register_sphere(KSphere {
                        center,
                        radius,
                        mat_type: mat.0,
                        mat_index: mat.1,
                    });
                }
            }
        }

        return kernel_scene;
    }
}
