#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use scene::types::WScene;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use utils::metrics::Metrics;

mod core;
mod output;
mod renderer;
mod scene;
mod utils;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn initialize_kernel() {
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn render(value: JsValue) {
    use scene::types::WScene;

    let scene = serde_wasm_bindgen::from_value::<WScene>(value).unwrap();

    wasm_bindgen_futures::spawn_local(run_internal(scene));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn render() {
    pollster::block_on(run_internal());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn parse_scene(value: JsValue) {
    use scene::types::WScene;

    let scene = serde_wasm_bindgen::from_value::<WScene>(value).unwrap();

    log::info!("{}", scene);
}

async fn run_internal(scene: WScene) {
    let mut metrics: Option<Metrics>;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            metrics = None;
        } else {
            metrics = Some(Metrics::new());
        }
    };

    if let Ok(buffer) = renderer::render(
        &scene.get_kernel_config(),
        &scene.get_kernel_scene(),
        &mut metrics,
    )
    .await
    {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                output::wasm::output_image(buffer, glam::uvec2(scene.render_settings.width, scene.render_settings.height));
            } else {
                output::native::output_image(buffer, glam::uvec2(scene.render_settings.width, scene.render_settings.height), "render.png");
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
