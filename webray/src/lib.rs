#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use utils::metrics::Metrics;

mod core;
mod demo;
mod output;
mod renderer;
mod scene;
mod utils;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn init() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().unwrap();
        } else {
            env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .format_timestamp_millis()
                .init();
        }
    }

    log::info!("WebRay Loaded");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_bindgen_futures::spawn_local(run_internal());
        } else {
            pollster::block_on(run_internal());
        }
    }
}

async fn run_internal() {
    let config = demo::create_cover_config();

    let scene = demo::create_cover_scene();

    let mut metrics: Option<Metrics>;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            metrics = None;
        } else {
            metrics = Some(Metrics::new());
        }
    };

    if let Ok(buffer) = renderer::render(&config, &scene.into(), &mut metrics).await {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                output::wasm::output_image(buffer, glam::uvec2(config.kernel.image.width, config.kernel.image.height));
            } else {
                output::native::output_image(buffer, glam::uvec2(config.kernel.image.width, config.kernel.image.height), "render.png");
            }
        }

        if let Some(m) = metrics.as_mut() {
            m.capture_output_write();
        }
    };

    if let Some(m) = metrics.as_mut() {
        m.capture_total();

        m.log();
    }
}
