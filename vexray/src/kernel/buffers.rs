use crate::core::gpu::Gpu;

use super::config::KernelConfig;

pub struct KernelBuffers {
    pub result: wgpu::Buffer,
    pub render: wgpu::Texture
}

impl KernelBuffers {
    pub fn new(gpu: &Gpu, config: &KernelConfig) -> Self {
        let render_texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });
    
        // &arr and &arr[..] are different, second one is a slice and what we need
        let result_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Result buffer"),
            size: config.size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return KernelBuffers {
            result: result_buffer,
            render: render_texture
        };
    }
}