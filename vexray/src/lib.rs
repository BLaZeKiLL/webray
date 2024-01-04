#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use log::{error, info};
use rand::Rng;
use renderer::config::{CameraConfig, KernelConfig, RenderConfig};
use scene::{material::Material, shape::Shape};
use utils::color;

mod core;
mod utils;
mod scene;
mod renderer;

pub async fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let path = "render.png";

    let config = create_cover_config();

    let scene = create_cover_scene();

    if let Ok(output) = renderer::render(&config, &scene.into()).await {
        output_image(output, &config, path);
    };
}

fn output_image(image_data: Vec<u8>, config: &KernelConfig, path: &str) {
    match image::save_buffer(
        path,
        &image_data,
        config.image.width,
        config.image.height,
        image::ColorType::Rgba8,
    ) {
        Ok(_) => info!("Image saved"),
        Err(e) => error!("{:?}", e),
    }
}

fn create_cover_config() -> KernelConfig {
    let render_config = RenderConfig {
        width: 1920,
        height: 1080,
        samples: 32,
        bounces: 8,
    };

    let camera_config = CameraConfig {
        look_from: glam::vec3(13.0, 2.0, 3.0),
        look_at: glam::vec3(0.0, 0.0, 0.0),
        v_up: glam::vec3(0.0, 1.0, 0.0),
        v_fov: 20.0,
        dof_angle: 0.6,
        dof_distance: 10.0,
    };

    return KernelConfig::new(render_config, camera_config);
}

fn create_cover_scene() -> scene::Scene {
    let mut scene = scene::Scene::new();
    let mut rng = rand::thread_rng();

    let ground_mat = scene.register_material(Material::Diffuse(glam::vec3(0.5, 0.5, 0.5)));
    scene.register_shape(Shape::Sphere(
        glam::vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = glam::vec3(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - glam::vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    let albedo = color::random_color(&mut rng) * color::random_color(&mut rng);
                    let mat = scene.register_material(Material::Diffuse(albedo));

                    scene.register_shape(Shape::Sphere(center, 0.2, mat));
                } else if choose_mat < 0.95 { // metal
                    let albedo = color::random_color_range(&mut rng, 0.5, 1.0);
                    let roughness: f32 = rng.gen_range(0.0..0.5);
                    let mat = scene.register_material(Material::Metal(albedo, roughness));

                    scene.register_shape(Shape::Sphere(center, 0.2, mat));
                } else { // dielectric
                    let mat = scene.register_material(Material::Dielectric(1.5));

                    scene.register_shape(Shape::Sphere(center, 0.2, mat));
                }
            }
        }
    }

    let mat1 = scene.register_material(Material::Dielectric(1.5));
    scene.register_shape(Shape::Sphere(glam::vec3(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = scene.register_material(Material::Diffuse(glam::vec3(0.4, 0.2, 0.1)));
    scene.register_shape(Shape::Sphere(glam::vec3(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = scene.register_material(Material::Metal(glam::vec3(0.7, 0.6, 0.5), 0.0));
    scene.register_shape(Shape::Sphere(glam::vec3(4.0, 1.0, 0.0), 1.0, mat3));

    return scene;
}

fn create_demo_config() -> KernelConfig {
    let render_config = RenderConfig {
        width: 1920,
        height: 1080,
        samples: 128,
        bounces: 16,
    };

    let camera_config = CameraConfig {
        look_from: glam::vec3(-2.0, 2.0, 1.0),
        look_at: glam::vec3(0.0, 0.0, -1.0),
        v_up: glam::vec3(0.0, 1.0, 0.0),
        v_fov: 20.0,
        dof_angle: 10.0,
        dof_distance: 3.4,
    };

    return KernelConfig::new(render_config, camera_config);
}

fn create_demo_scene() -> scene::Scene {
    let mut scene = scene::Scene::new();

    let diffuse_mat_1 = scene.register_material(Material::Diffuse(glam::vec3(0.6, 0.8, 0.0)));

    let diffuse_mat_2 = scene.register_material(Material::Diffuse(glam::vec3(0.1, 0.2, 0.5)));

    let metal_mat_2 = scene.register_material(Material::Metal(glam::vec3(0.8, 0.6, 0.2), 0.1));

    let dielectric_mat_1 = scene.register_material(Material::Dielectric(1.5));

    // left
    // 2 spheres and 1 with -ve radius with di-electric mat gives a hollow glass bubble look
    scene.register_shape(Shape::Sphere(
        glam::vec3(-1.0, 0.0, -1.0),
        0.5,
        dielectric_mat_1,
    ));
    scene.register_shape(Shape::Sphere(
        glam::vec3(-1.0, 0.0, -1.0),
        -0.4,
        dielectric_mat_1,
    ));

    // center
    scene.register_shape(Shape::Sphere(
        glam::vec3(0.0, 0.0, -1.0),
        0.5,
        diffuse_mat_2,
    ));

    // right
    scene.register_shape(Shape::Sphere(glam::vec3(1.0, 0.0, -1.0), 0.5, metal_mat_2));

    // ground
    scene.register_shape(Shape::Sphere(
        glam::vec3(0.0, -100.5, -1.0),
        100.0,
        diffuse_mat_1,
    ));

    return scene;
}
