#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use log::{error, info};

mod core;
mod renderer;

pub async fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let path = "render.png";
    let config = renderer::config::KernelConfig::new(1920, 1080);

    let mut world = renderer::world::World::new();

    world.add_sphere(glam::vec3(0.0, 0.0, -1.0), 0.5);
    world.add_sphere(glam::vec3(0.0, -100.5, -1.0), 100.0);

    if let Ok(output) = renderer::render(&config, &world).await {
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
