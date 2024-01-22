#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use renderer::config::Config;
use utils::metrics::Metrics;

mod core;
mod demo;
mod renderer;
mod scene;
mod utils;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Could not initialize logger");
            wasm_bindgen_futures::spawn_local(run_internal());
        } else {
            env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .format_timestamp_millis()
                .init();

            pollster::block_on(run_internal());
        }
    }
}

async fn run_internal() {
    let path = "render.png";

    let config = demo::create_demo_config();

    let scene = demo::create_demo_scene();

    let mut metrics = Some(Metrics::new());

    if let Ok(output) = renderer::render(&config, &scene.into(), &mut metrics).await {
        output_image(output, &config, path);

        if let Some(m) = metrics.as_mut() {
            m.capture_output_write();
        }
    };

    if let Some(m) = metrics.as_mut() {
        m.capture_total();

        m.log();
    }
}

fn output_image(image_data: Vec<u8>, config: &Config, path: &str) {
    match image::save_buffer(
        path,
        &image_data,
        config.kernel.image.width,
        config.kernel.image.height,
        image::ColorType::Rgba8,
    ) {
        Ok(_) => log::info!("Output saved at path: {}", path),
        Err(e) => log::error!("{:?}", e),
    }
}
