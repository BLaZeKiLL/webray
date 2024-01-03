use crate::core::gpu::Gpu;

use super::buffers::KernelBuffers;

pub struct KernelBindings {
    pub config_layout: wgpu::BindGroupLayout,
    pub config_binding: Option<wgpu::BindGroup>,
    pub scene_layout: wgpu::BindGroupLayout,
    pub scene_binding: Option<wgpu::BindGroup>,
    pub material_layout: wgpu::BindGroupLayout,
    pub material_binding: Option<wgpu::BindGroup>,
}

impl KernelBindings {
    pub fn new(gpu: &Gpu) -> Self {
        let config_layout = Self::config_bind_group_layout(gpu);
        let scene_layout = Self::scene_bind_group_layout(gpu);
        let material_layout = Self::material_bind_group_layout(gpu);

        return KernelBindings {
            config_layout,
            config_binding: None,
            scene_layout,
            scene_binding: None,
            material_layout,
            material_binding: None,
        };
    }

    pub fn bind_buffers(&mut self, gpu: &Gpu, buffers: &KernelBuffers) {
        self.config_binding = Some(self.config_bind_group(gpu, buffers));
        self.scene_binding = Some(self.scene_bind_group(gpu, buffers));
        self.material_binding = Some(self.material_bind_group(gpu, buffers));
    }

    pub fn pipeline_layout(&self) -> Vec<&wgpu::BindGroupLayout> {
        let layout = vec![
            &self.config_layout,
            &self.scene_layout,
            &self.material_layout,
        ];

        return layout;
    }

    fn config_bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
        return gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Config bind group layout"),
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
                ],
            });
    }

    fn config_bind_group(&self, gpu: &Gpu, buffers: &KernelBuffers) -> wgpu::BindGroup {
        return gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Config bind group"),
            layout: &self.config_layout,
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

    fn scene_bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
        return gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Scene bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
    }

    fn scene_bind_group(&self, gpu: &Gpu, buffers: &KernelBuffers) -> wgpu::BindGroup {
        return gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene bind group"),
            layout: &self.scene_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffers.spheres.as_entire_binding(),
            }],
        });
    }

    fn material_bind_group_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
        return gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Material bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
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
                ],
            });
    }

    fn material_bind_group(&self, gpu: &Gpu, buffers: &KernelBuffers) -> wgpu::BindGroup {
        return gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material bind group"),
            layout: &self.material_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffers.diffuse_mats.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffers.metal_mats.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: buffers.dielectric_mats.as_entire_binding()
                }
            ],
        });
    }
}
