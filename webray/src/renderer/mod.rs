use log::{error, info};

use crate::{
    core::gpu::Gpu,
    renderer::{bindings::KernelBindings, buffers::KernelBuffers, kernel::Kernel},
};

use self::{
    config::Config,
    scene::KernelScene,
};

mod bindings;
mod buffers;
mod kernel;

pub mod config;
pub mod material;
pub mod scene;
pub mod shapes;

pub async fn render(config: &Config, scene: &KernelScene) -> Result<Vec<u8>, ()> {
    // dbg!(&config);
    // dbg!(&scene);

    info!("Render start");

    let gpu = Gpu::new().await;

    info!("Device acquired");

    let buffers = KernelBuffers::new(&gpu, &config.kernel, scene);

    let mut bindings = KernelBindings::new(&gpu);

    bindings.bind_buffers(&gpu, &buffers);

    info!("Scene buffers uploaded");

    let kernel = Kernel::new(&gpu, &bindings);

    info!("Kernel initialized");

    let result = kernel
        .execute(&gpu, &config.kernel, &config.system, &bindings, &buffers)
        .await;

    match result {
        Ok(_) => info!("Render finished"),
        Err(_) => error!("Something went wrong"),
    }

    return result;
}
