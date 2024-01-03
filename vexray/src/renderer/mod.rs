use log::{error, info};

use crate::{core::gpu::Gpu, renderer::{buffers::KernelBuffers, kernel::Kernel, bindings::KernelBindings}};

use self::{config::KernelConfig, scene::KernelScene, };

mod kernel;
mod buffers;
mod bindings;

pub mod material;
pub mod config;
pub mod shapes;
pub mod scene;

pub async fn render(config: &KernelConfig, scene: &KernelScene) -> Result<Vec<u8>, ()> {
    dbg!(&scene);

    info!("Render start");

    let gpu = Gpu::new().await;

    info!("Device acquired");

    let buffers = KernelBuffers::new(&gpu, config, scene);
    let mut bindings = KernelBindings::new(&gpu);

    bindings.bind_buffers(&gpu, &buffers);

    info!("Scene buffers uploaded");

    let kernel = Kernel::new(&gpu, &bindings);

    info!("Kernel initialized");

    kernel.submit(&gpu, config, &bindings, &buffers);

    info!("Commands submitted");

    let result = kernel.finish(&gpu, config, &buffers).await;

    match result {
        Ok(_) => info!("Render finished"),
        Err(_) => error!("Something went wrong"),
    }

    return result;
}