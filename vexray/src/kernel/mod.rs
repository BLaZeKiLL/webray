use log::{error, info};

use crate::{core::gpu::Gpu, kernel::buffers::KernelBuffers};

use self::config::KernelConfig;

mod buffers;
pub mod config;

pub async fn render(config: &KernelConfig) -> Result<Vec<u8>, ()> {
    info!("Render start");

    let gpu = Gpu::new().await;

    let pipeline = create_pipeline(&gpu);

    let buffers = KernelBuffers::new(&gpu, config);

    let bind_group = create_bind_group(&gpu, &pipeline, &buffers);

    info!("Render kernel initialized");

    submit(&gpu, config, &pipeline, &buffers, &bind_group);

    info!("Render commands submitted");

    return finish(&gpu, config, &buffers).await;
}

/// Creates the compute pipeline and imports the vexray.wgsl compute shader
fn create_pipeline(gpu: &Gpu) -> wgpu::ComputePipeline {
    let shader = gpu
        .device
        .create_shader_module(wgpu::include_wgsl!("../shaders/vexray.wgsl"));

    return gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute pipeline"),
            layout: None,
            module: &shader,
            entry_point: "main",
        });
}

/// Creates the bind group
/// binds render texture to group(0), binding(0)
fn create_bind_group(
    gpu: &Gpu,
    pipeline: &wgpu::ComputePipeline,
    buffers: &KernelBuffers,
) -> wgpu::BindGroup {
    let bind_group_layout = pipeline.get_bind_group_layout(0);

    return gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute bind group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(
                    &buffers
                        .render
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: buffers.config.as_entire_binding(),
            },
        ],
    });
}

/// Creates the command encoders and configures the compute pass
/// Submits the commands to the gpu
fn submit(
    gpu: &Gpu,
    config: &KernelConfig,
    pipeline: &wgpu::ComputePipeline,
    buffers: &KernelBuffers,
    bind_group: &wgpu::BindGroup,
) {
    // Setup commands
    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command encoder"),
        });

    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute pass"),
            timestamp_writes: None,
        });

        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.dispatch_workgroups(config.image.width, config.image.height, 1);
    }

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &buffers.render,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &buffers.result,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(config.image.width * 4),
                rows_per_image: Some(config.image.height),
            },
        },
        wgpu::Extent3d {
            width: config.image.width,
            height: config.image.height,
            depth_or_array_layers: 1,
        },
    );

    // Submit commands
    gpu.queue.submit([encoder.finish()]);
}

/// waits for finish by using device.poll (block on main thread)
/// returns the data from result buffer
async fn finish(gpu: &Gpu, config: &KernelConfig, buffers: &KernelBuffers) -> Result<Vec<u8>, ()> {
    let mut output = vec![0u8; (config.result_size()) as usize];

    let result_slice = buffers.result.slice(..);

    let (sender, receiver) = flume::bounded(1);

    result_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    // Wait for result
    gpu.device.poll(wgpu::Maintain::Wait);

    if let Ok(Ok(_)) = receiver.recv_async().await {
        let result_view = result_slice.get_mapped_range();

        output.copy_from_slice(&result_view[..]);
    } else {
        error!("Something went wrong");

        return Err(());
    }

    // Cleanup
    // result view would be dropped by here
    buffers.result.unmap();

    info!("Render finished");

    return Ok(output);
}
