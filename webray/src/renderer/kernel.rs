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

    pub async fn execute(
        &self,
        gpu: &Gpu,
        config: &KernelConfig,
        bindings: &KernelBindings,
        buffers: &KernelBuffers,
    ) -> Result<Vec<u8>, ()> {
        let (sender, receiver) = flume::bounded(1);

        // scoped threads can borrow non-'static data as scope guarantees
        // all threads will join at the end of the scope
        std::thread::scope(|scope| {
            let _ = scope.spawn(move || {
                let submission_index = self.submit(gpu, config, bindings, buffers);
    
                self.wait(gpu, buffers, submission_index, sender);
            });
        });

        let result = self.finish(config, buffers, receiver).await;

        return result;
    }

    fn submit(
        &self,
        gpu: &Gpu,
        config: &KernelConfig,
        bindings: &KernelBindings,
        buffers: &KernelBuffers,
    ) -> wgpu::SubmissionIndex {
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
        return gpu.queue.submit([encoder.finish()]);
    }

    fn wait(        
        &self,
        gpu: &Gpu,
        buffers: &KernelBuffers,
        submission_index: wgpu::SubmissionIndex,
        sender: flume::Sender<Result<(), wgpu::BufferAsyncError>>,
    ) {
        let result_slice = buffers.result.slice(..);

        result_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Wait for result
        // wgpu has an internal timeout of 5 secs
        // wgpu has a bug where timeout is treated as success which triggers the map_async callback
        // causing early mapping when without the results being populated
        // https://github.com/gfx-rs/wgpu/issues/3601
        gpu.device.poll(wgpu::Maintain::WaitForSubmissionIndex(submission_index));
    }

    async fn finish(
        &self,
        config: &KernelConfig,
        buffers: &KernelBuffers,
        receiver: flume::Receiver<Result<(), wgpu::BufferAsyncError>>
    ) -> Result<Vec<u8>, ()> {
        let mut output = vec![0u8; (config.result_size()) as usize];

        if let Ok(Ok(_)) = receiver.recv_async().await {
            let result_view = buffers.result.slice(..).get_mapped_range();

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
            .create_shader_module(wgpu::include_wgsl!("../shaders/webray.wgsl"));

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
