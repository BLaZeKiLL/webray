#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use core::gpu::Gpu;

use log::{error, info};
use wgpu::util::DeviceExt;

mod core;

pub async fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    run_compute_double().await;
}

async fn run_compute_double() {
    let numbers = [1, 2, 3, 4, 5];

    let gpu = Gpu::new().await;

    let result = compute_double(&gpu, &numbers).await.unwrap();

    let output: Vec<String> = result.iter().map(|&n| return n.to_string()).collect();

    info!("Result: [{}]", output.join(", "));
}

async fn compute_double(gpu: &Gpu, numbers: &[i32]) -> Option<Vec<i32>> {
    // This shader doubles the number in place in work_buffer
    // but to read result we need to copy it to result buffer
    // see the fundamentals to understand why this copy is required
    // https://webgpufundamentals.org/webgpu/lessons/webgpu-fundamentals.html#a-run-computations-on-the-gpu
    let shader = gpu
        .device
        .create_shader_module(wgpu::include_wgsl!("./shaders/double.wgsl"));

    let size = std::mem::size_of_val(numbers) as wgpu::BufferAddress; // cast is required, idk why

    // create buffer without any data
    let result_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Result buffer"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // create buffer with data, this will write the date to buffer also similar to calling queue.write_buffer
    let work_buffer = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Work buffer"),
            contents: bytemuck::cast_slice(numbers),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
        });

    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute pipeline"),
            layout: None,
            module: &shader,
            entry_point: "main",
        });

    // get the layout of bind group 0 from the shader
    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute bind group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry { // set work_buffer as binding 0 in this group
            binding: 0,
            resource: work_buffer.as_entire_binding(),
        }],
    });

    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command encoder"),
        });

    { // compute pass, scoped cause otherwise drop pass manually
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute pass"),
            timestamp_writes: None,
        });

        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.insert_debug_marker("Compute double");
        pass.dispatch_workgroups(numbers.len() as u32, 1, 1);
    }

    encoder.copy_buffer_to_buffer(&work_buffer, 0, &result_buffer, 0, size);

    gpu.queue.submit([encoder.finish()]);

    // slice is the internal data i guess
    let result_slice = result_buffer.slice(..);

    // we use channel as a notification as callback is called when buffer is ready to be mapped
    let (sender, receiver) = flume::bounded(1);

    result_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    gpu.device.poll(wgpu::Maintain::Wait);

    if let Ok(Ok(_)) = receiver.recv_async().await {
        let data = result_slice.get_mapped_range();

        let result = bytemuck::cast_slice(&data).to_vec();

        drop(data);
        result_buffer.unmap();

        return Some(result);
    } else {
        error!("Something went wrong");
        return None;
    }
}
