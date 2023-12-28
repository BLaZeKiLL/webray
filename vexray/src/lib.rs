#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use log::{error, info};

mod core;
mod kernel;

pub async fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let config = kernel::config::KernelConfig::new(1920, 1080);
    let path = "render.png";

    if let Ok(output) = kernel::render(&config).await {
        output_image(output, &config, path);
    };
}

fn output_image(image_data: Vec<u8>, config: &kernel::config::KernelConfig, path: &str) {
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
