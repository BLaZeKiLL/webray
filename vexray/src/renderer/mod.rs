use log::{error, info};

use crate::{core::gpu::Gpu, renderer::{buffers::KernelBuffers, kernel::Kernel}};

use self::{config::KernelConfig, world::World};

mod kernel;
mod buffers;
mod shapes;

pub mod config;
pub mod world;

pub async fn render(config: &KernelConfig, world: &World) -> Result<Vec<u8>, ()> {
    info!("Render start");

    let gpu = Gpu::new().await;

    info!("Device acquired");

    let buffers = KernelBuffers::new(&gpu, config, world);

    info!("Scene buffers uploaded");

    let kernel = Kernel::new(&gpu, &buffers);

    info!("Kernel initialized");

    kernel.submit(&gpu, config, &buffers);

    info!("Commands submitted");

    let result = kernel.finish(&gpu, config, &buffers).await;

    match result {
        Ok(_) => info!("Render finished"),
        Err(_) => error!("Something went wrong"),
    }

    return result;
}