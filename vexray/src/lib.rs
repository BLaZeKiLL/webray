#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use log::{error, info};
use renderer::config::{CameraConfig, KernelConfig, RenderConfig};
use scene::{material::Material, shape::Shape};

mod core;
mod renderer;
mod scene;

pub async fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let path = "render.png";

    let render_config = RenderConfig {
        width: 1920,
        height: 1080,
        samples: 128,
        bounces: 16,
    };

    let camera_config = CameraConfig {
        look_from: glam::vec3(-2.0, 2.0, 1.0),
        look_at: glam::vec3(0.0, 0.0, -1.0),
        vup: glam::vec3(0.0, 1.0, 0.0),
        vfov: 90.0,
    };
    let config = KernelConfig::new(render_config, camera_config);

    let scene = create_demo_scene();

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

fn create_demo_scene() -> scene::Scene {
    let mut scene = scene::Scene::new();

    let diffuse_mat_1 = scene.register_material(Material::Diffuse(glam::vec3(0.6, 0.8, 0.0)));

    let diffuse_mat_2 = scene.register_material(Material::Diffuse(glam::vec3(0.1, 0.2, 0.5)));

    // let metal_mat_1 = scene.register_material(Material::Metal(glam::vec3(0.8, 0.8, 0.8), 0.2));

    let metal_mat_2 = scene.register_material(Material::Metal(glam::vec3(0.8, 0.6, 0.8), 0.8));

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
