use wgpu::util::DeviceExt;

use crate::core::gpu::Gpu;

use super::{config::KernelConfig, scene::KernelScene};

/// Container for kernel buffers
pub struct KernelBuffers {
    /// output from render texture is copied to this
    /// so that it can be mapped and read
    pub result: wgpu::Buffer,
    /// storage texture where the rendered image is
    /// written in the compute shader
    pub render: wgpu::Texture,
    /// kernel config uniform buffer
    pub config: wgpu::Buffer,

    // Scene buffers
    pub spheres: wgpu::Buffer,
    pub diffuse_mats: wgpu::Buffer,
    pub metal_mats: wgpu::Buffer,
    pub dielectric_mats: wgpu::Buffer,
}

impl KernelBuffers {
    pub fn new(gpu: &Gpu, config: &KernelConfig, scene: &KernelScene) -> Self {
        // &arr and &arr[..] are different, second one is a slice and what we need
        let result_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Result buffer"),
            size: config.result_size(),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let render_texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render texture"),
            size: wgpu::Extent3d {
                width: config.image.width,
                height: config.image.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm, // format is specified in the shader
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });

        // using buffer init and write the buffer here it self, don't need to do queue.write_buffer
        // maybe we can delay this, lazy upload

        let config_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Config uniform buffer"),
                contents: &config.as_wgsl_bytes().unwrap()[..],
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let spheres_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Scene spheres buffer"),
                contents: &scene.spheres_as_wgsl_bytes().unwrap()[..],
                usage: wgpu::BufferUsages::STORAGE,
            });

        let diffuse_mats_buffer =
            gpu.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Scene diffuse materials buffer"),
                    contents: &scene.diffuse_mats_as_wgsl_bytes().unwrap()[..],
                    usage: wgpu::BufferUsages::STORAGE,
                });

        let metal_mats_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Scene metal materials buffer"),
                contents: &scene.metal_mats_as_wgsl_bytes().unwrap()[..],
                usage: wgpu::BufferUsages::STORAGE,
            });

        let dielectric_mats_buffer =
            gpu.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Scene dielectric materials buffer"),
                    contents: &scene.dielectric_mats_as_wgsl_bytes().unwrap()[..],
                    usage: wgpu::BufferUsages::STORAGE,
                });

        return KernelBuffers {
            result: result_buffer,
            render: render_texture,
            config: config_buffer,
            spheres: spheres_buffer,
            diffuse_mats: diffuse_mats_buffer,
            metal_mats: metal_mats_buffer,
            dielectric_mats: dielectric_mats_buffer
        };
    }
}
