#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use log::{error, info};

mod core;
mod scene;
mod renderer;

pub async fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let path = "render.png";
    let config = renderer::config::KernelConfig::new(1920, 1080);

    let scene = create_demo_scene();

    if let Ok(output) = renderer::render(&config, &scene.into()).await {
        output_image(output, &config, path);
    };
}

fn output_image(image_data: Vec<u8>, config: &renderer::config::KernelConfig, path: &str) {
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
    
    let diffuse_mat_1 = scene.register_material(scene::material::Material::Diffuse(glam::vec3(0.8, 0.8, 0.0)));
    let diffuse_mat_2 = scene.register_material(scene::material::Material::Diffuse(glam::vec3(0.8, 0.8, 0.0)));

    let sphere_1 = scene::shape::Shape::Sphere(glam::vec3(0.0, 0.0, -1.0), 0.5, diffuse_mat_1);
    let sphere_2 = scene::shape::Shape::Sphere(glam::vec3(0.0, -100.5, -1.0), 100.0, diffuse_mat_2);

    scene.register_shape(sphere_1);
    scene.register_shape(sphere_2);

    return scene;
}