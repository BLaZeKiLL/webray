use crate::core::gpu::Gpu;

use super::buffers::KernelBuffers;

pub struct KernelBindings {
    pub system_layout: wgpu::BindGroupLayout,
    pub system_binding: Option<wgpu::BindGroup>,
    pub user_layout: wgpu::BindGroupLayout,
    pub user_binding: Option<wgpu::BindGroup>,
    pub execution_layout: wgpu::BindGroupLayout,
    pub execution_binding: Option<wgpu::BindGroup>,
}

impl KernelBindings {
    pub fn new(gpu: &Gpu) -> Self {
        let config_layout = Self::system_bind_group_layout(gpu);
        let scene_layout = Self::user_bind_group_layout(gpu);
        let material_layout = Self::execution_bind_group_layout(gpu);

        return KernelBindings {
            system_layout: config_layout,
            system_binding: None,
            user_layout: scene_layout,
            user_binding: None,
            execution_layout: material_layout,
            execution_binding: None,
        };
    }

    pub fn bind_buffers(&mut self, gpu: &Gpu, buffers: &KernelBuffers) {
        self.system_binding = Some(self.system_bind_group(gpu, buffers));
        self.user_binding = Some(self.user_bind_group(gpu, buffers));
        self.execution_binding = Some(self.execution_bind_group(gpu, buffers));
    }

    pub fn pipeline_layout(&self) -> Vec<&wgpu::BindGroupLayout> {
        let layout = vec![
            &self.system_layout,
            &self.user_layout,
            &self.execution_layout,
        ];

        return layout;
    }

    fn system_bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
        return gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("System bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                }],
            });
    }

    fn system_bind_group(&self, gpu: &Gpu, buffers: &KernelBuffers) -> wgpu::BindGroup {
        return gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("System bind group"),
            layout: &self.system_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(
                    &buffers
                        .render
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            }],
        });
    }

    fn user_bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
        return gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("User bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
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
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
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

    fn user_bind_group(&self, gpu: &Gpu, buffers: &KernelBuffers) -> wgpu::BindGroup {
        return gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene bind group"),
            layout: &self.user_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffers.config.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffers.spheres.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: buffers.diffuse_mats.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: buffers.metal_mats.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: buffers.dielectric_mats.as_entire_binding(),
                },
            ],
        });
    }

    fn execution_bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
        return gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Execution bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
    }

    fn execution_bind_group(&self, gpu: &Gpu, buffers: &KernelBuffers) -> wgpu::BindGroup {
        return gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Execution bind group"),
            layout: &self.execution_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffers.execution_context.as_entire_binding(),
            }],
        });
    }
}
