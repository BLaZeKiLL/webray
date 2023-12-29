use crate::core::gpu::Gpu;

use super::{buffers::KernelBuffers, config::KernelConfig};

pub struct Kernel {
    pub pipeline: wgpu::ComputePipeline,
    pub bind_group: wgpu::BindGroup,
}

impl Kernel {
    pub fn new(gpu: &Gpu, buffers: &KernelBuffers) -> Self {
        let bind_group_layout = Kernel::create_bind_group_layout(gpu);

        let bind_group = Kernel::create_bind_group(gpu, &bind_group_layout, buffers);
        let pipeline = Kernel::create_pipeline(gpu, &bind_group_layout);

        return Kernel {
            pipeline,
            bind_group,
        };
    }

    pub fn submit(&self, gpu: &Gpu, config: &KernelConfig, buffers: &KernelBuffers) {
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
            pass.set_bind_group(0, &self.bind_group, &[]);
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
        gpu.device.poll(wgpu::Maintain::Wait);

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

    fn create_bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
        return gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Compute bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::Rgba8Unorm,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
    }

    fn create_pipeline(
        gpu: &Gpu,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::ComputePipeline {
        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("../shaders/vexray.wgsl"));

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute pipeline layout"),
                bind_group_layouts: &[bind_group_layout],
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

    fn create_bind_group(
        gpu: &Gpu,
        bind_group_layout: &wgpu::BindGroupLayout,
        buffers: &KernelBuffers,
    ) -> wgpu::BindGroup {
        return gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute bind group"),
            layout: bind_group_layout,
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
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: buffers.spheres.as_entire_binding(),
                },
            ],
        });
    }
}
