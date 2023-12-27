use crate::core::gpu::Gpu;

use log::info;

pub struct App {
    gpu: Gpu,
    pipeline: Option<wgpu::RenderPipeline>,
}

impl App {
    pub fn new(gpu: Gpu) -> Self {
        return App {
            gpu,
            pipeline: None,
        };
    }

    pub fn start(&mut self) {
        info!("App start");

        let shader = self
            .gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("../shaders/hello.wgsl"));

        let layout = self
            .gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("App render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let pipeline = self
            .gpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("App render pipeline"),
                layout: Some(&layout), // none may be auto
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs",
                    targets: &[Some(self.gpu.config.format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        self.pipeline = Some(pipeline);
    }

    pub fn input(&self, _event: &winit::event::WindowEvent) -> bool {
        return false;
    }

    pub fn update(&self) {}

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.gpu.surface.get_current_texture().unwrap();

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("App command encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("App render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(self.pipeline.as_ref().unwrap());
            pass.draw(0..3, 0..1);
        }

        self.gpu.queue.submit([encoder.finish()]);

        frame.present();

        return Ok(());
    }

    pub fn destroy(&mut self) {
        info!("App destroy");
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.gpu.resize(new_size);
    }
}
