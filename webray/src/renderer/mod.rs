use crate::{
    core::gpu::Gpu,
    renderer::{bindings::KernelBindings, buffers::KernelBuffers, kernel::Kernel},
    utils::metrics::Metrics,
};

use self::{config::KernelConfig, scene::KernelScene};

mod bindings;
mod buffers;
mod kernel;

pub mod config;
pub mod material;
pub mod scene;
pub mod shapes;

pub async fn render(
    config: &KernelConfig,
    scene: &KernelScene,
    metrics: &mut Option<Metrics>,
) -> Result<Vec<u8>, ()> {
    // dbg!(&config);
    // dbg!(&scene);

    log::info!("Render start");

    if let Some(m) = metrics.as_mut() {
        m.start();
    }

    let gpu = Gpu::new().await;

    log::info!("Device acquired");

    if let Some(m) = metrics.as_mut() {
        m.capture_device_acquisition();
    }

    let buffers = KernelBuffers::new(&gpu, &config.system, scene);

    let mut bindings = KernelBindings::new(&gpu);

    bindings.bind_buffers(&gpu, &buffers);

    log::info!("Scene buffers uploaded");

    if let Some(m) = metrics.as_mut() {
        m.capture_scene_upload();
    }

    let kernel = Kernel::new(&gpu, &bindings);

    log::info!("Kernel initialized");

    if let Some(m) = metrics.as_mut() {
        m.capture_kernel_initialization();
    }

    let result = kernel
        .execute(&gpu, &config.system, &config.execution, &bindings, &buffers)
        .await;

    match result {
        Ok(_) => log::info!("Render finished"),
        Err(_) => log::error!("Something went wrong"),
    }

    if let Some(m) = metrics.as_mut() {
        m.capture_rendering();
    }

    return result;
}
