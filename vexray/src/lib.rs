#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use core::gpu::Gpu;

use log::{error, info};

mod core;

pub async fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    let dims: (u32, u32) = (1920, 1080);

    if let Ok(output) = render(dims).await {
        output_image(output, dims, "render.png")
    };
}

async fn render(dims: (u32, u32)) -> Result<Vec<u8>, ()> {
    let width: u32 = dims.0;
    let height: u32 = dims.1;

    info!("Render start: ({}, {})", width, height);

    let gpu = Gpu::new().await;

    // byte buffer to which final data would be written
    let mut output_data = vec![0u8; (width * height * 4) as usize];

    let shader = gpu
        .device
        .create_shader_module(wgpu::include_wgsl!("./shaders/vexray.wgsl"));

    let render_texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render texture"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
    });

    let render_texture_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());

    // &arr and &arr[..] are different, second one is a slice and what we need
    let result_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Result buffer"),
        size: std::mem::size_of_val(&output_data[..]) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute pipeline"),
            layout: None,
            module: &shader,
            entry_point: "main",
        });

    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute bind group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            // set work_buffer as binding 0 in this group
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&render_texture_view),
        }],
    });

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

        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(width, height, 1);
    }

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &render_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &result_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    gpu.queue.submit([encoder.finish()]);

    let result_slice = result_buffer.slice(..);

    let (sender, receiver) = flume::bounded(1);

    result_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    gpu.device.poll(wgpu::Maintain::Wait);

    if let Ok(Ok(_)) = receiver.recv_async().await {
        let result_view = result_slice.get_mapped_range();

        output_data.copy_from_slice(&result_view[..]);
    } else {
        error!("Something went wrong");

        return Err(());
    }

    // result view would be dropped by here
    result_buffer.unmap();

    info!("Render complete");

    return Ok(output_data);
}

fn output_image(image_data: Vec<u8>, dims: (u32, u32), path: &str) {
    match image::save_buffer(path, &image_data, dims.0, dims.1, image::ColorType::Rgba8) {
        Ok(_) => info!("Image saved"),
        Err(e) => error!("{:?}", e),
    }
}