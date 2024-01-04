use log::info;

use crate::core::gpu::Gpu;

use super::{bindings::KernelBindings, buffers::KernelBuffers, config::KernelConfig};

pub struct Kernel {
    pub pipeline: wgpu::ComputePipeline,
}

impl Kernel {
    pub fn new(gpu: &Gpu, bindings: &KernelBindings) -> Self {
        let pipeline = Kernel::create_pipeline(gpu, bindings);

        return Kernel { pipeline };
    }

    pub fn submit(
        &self,
        gpu: &Gpu,
        config: &KernelConfig,
        bindings: &KernelBindings,
        buffers: &KernelBuffers,
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

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, bindings.config_binding.as_ref().unwrap(), &[]);
            pass.set_bind_group(1, bindings.scene_binding.as_ref().unwrap(), &[]);
            pass.set_bind_group(2, bindings.material_binding.as_ref().unwrap(), &[]);
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

    pub async fn finish(
        &self,
        gpu: &Gpu,
        config: &KernelConfig,
        buffers: &KernelBuffers,
    ) -> Result<Vec<u8>, ()> {
        let mut output = vec![0u8; (config.result_size()) as usize];

        let result_slice = buffers.result.slice(..);

        let (sender, receiver) = flume::bounded(1);

        result_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Wait for result
        let status = gpu.device.poll(wgpu::Maintain::Wait);

        info!("Poll status: {}", status);

        if let Ok(Ok(_)) = receiver.recv_async().await {
            let result_view = result_slice.get_mapped_range();

            output.copy_from_slice(&result_view[..]);
        } else {
            return Err(());
        }

        // Cleanup
        // result view would be dropped by here
        buffers.result.unmap();

        return Ok(output);
    }

    fn create_pipeline(gpu: &Gpu, bindings: &KernelBindings) -> wgpu::ComputePipeline {
        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("../shaders/vexray.wgsl"));

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute pipeline layout"),
                bind_group_layouts: &bindings.pipeline_layout()[..],
                push_constant_ranges: &[],
            });

        return gpu
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Compute pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point: "main",
            });
    }
}
