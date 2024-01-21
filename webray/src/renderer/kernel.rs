use crate::core::gpu::Gpu;

use super::{
    bindings::KernelBindings,
    buffers::KernelBuffers,
    config::{ExecutionContext, KernelConfig, SystemConfig, TileSize},
};

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
        kernel_config: &KernelConfig,
        system_config: &SystemConfig,
        bindings: &KernelBindings,
        buffers: &KernelBuffers,
    ) -> Result<Vec<u8>, ()> {
        let tile_count = Self::count_tiles(
            kernel_config.image.width,
            kernel_config.image.height,
            system_config.tile_size,
        );

        log::info!(
            "Tile count: {}, width: {}, height: {}",
            tile_count.x * tile_count.y,
            kernel_config.image.width,
            kernel_config.image.height
        );

        match system_config.tile_size {
            TileSize::Full => {
                log::info!(
                    "Rendering full tile, width: {}, height: {}",
                    kernel_config.image.width,
                    kernel_config.image.height
                );

                self.render_tile(
                    gpu,
                    glam::uvec2(0, 0),
                    kernel_config.image.width,
                    kernel_config.image.height,
                    bindings,
                    buffers,
                );
            }
            TileSize::Square(size) => {
                let mut id = 1;
                for x in 0..tile_count.x {
                    for y in 0..tile_count.y {
                        let tile_position = glam::uvec2(x, y);

                        let width = ((x + 1) * size).min(kernel_config.image.width) - (x * size);
                        let height = ((y + 1) * size).min(kernel_config.image.height) - (y * size);

                        log::info!(
                            "Rendering tile {}: {}, width: {}, height: {}",
                            id,
                            tile_position,
                            width,
                            height
                        );

                        self.render_tile(gpu, tile_position * size, width, height, bindings, buffers);

                        id += 1;
                    }
                }
            }
        }

        // scoped threads can borrow non-'static data as scope guarantees
        // all threads will join at the end of the scope
        // std::thread::scope(|scope| {
        //     let _ = scope.spawn(move || {

        //     });
        // });

        log::info!("Reading result buffer");

        let result = self.map_result(gpu, kernel_config, buffers).await;

        return result;
    }

    fn render_tile(
        &self,
        gpu: &Gpu,
        tile_position: glam::UVec2,
        width: u32,
        height: u32,
        bindings: &KernelBindings,
        buffers: &KernelBuffers,
    ) {
        // Write execution context
        let execution_context = ExecutionContext { tile_position };

        gpu.queue.write_buffer(
            &buffers.execution_context,
            0,
            &execution_context.as_wgsl_bytes().unwrap()[..],
        );

        // Setup commands
        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        {
            // scope to bypass mutable borrow
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute pass"),
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, bindings.system_binding.as_ref().unwrap(), &[]);
            pass.set_bind_group(1, bindings.user_binding.as_ref().unwrap(), &[]);
            pass.set_bind_group(2, bindings.execution_binding.as_ref().unwrap(), &[]);
            pass.dispatch_workgroups(width, height, 1);
        }

        // Submit commands
        let submission_index = gpu.queue.submit([encoder.finish()]);

        gpu.device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(submission_index))
            .panic_on_timeout();
    }

    async fn map_result(
        &self,
        gpu: &Gpu,
        kernel_config: &KernelConfig,
        buffers: &KernelBuffers,
    ) -> Result<Vec<u8>, ()> {
        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Result encoder"),
            });

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
                    bytes_per_row: Some(kernel_config.image.width * 4),
                    rows_per_image: Some(kernel_config.image.height),
                },
            },
            wgpu::Extent3d {
                width: kernel_config.image.width,
                height: kernel_config.image.height,
                depth_or_array_layers: 1,
            },
        );

        let submission_index = gpu.queue.submit([encoder.finish()]);

        let (sender, receiver) = flume::bounded(1);

        let result_slice = buffers.result.slice(..);

        result_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Wait for result
        // wgpu has an internal timeout of 5 secs
        // wgpu has a bug where timeout is treated as success which triggers the map_async callback
        // causing early mapping when without the results being populated
        // https://github.com/gfx-rs/wgpu/issues/3601
        gpu.device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(submission_index))
            .panic_on_timeout();

        let mut output = vec![0u8; (kernel_config.result_size()) as usize];

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

    fn count_tiles(width: u32, height: u32, tile_size: TileSize) -> glam::UVec2 {
        return match tile_size {
            TileSize::Full => glam::uvec2(1, 1),
            TileSize::Square(size) => glam::uvec2(width.div_ceil(size), height.div_ceil(size)),
        };
    }
}
