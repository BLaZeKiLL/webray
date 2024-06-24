pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Gpu {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::default();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Main GPU"),
                    required_features: 
                        wgpu::Features::default() | 
                        wgpu::Features::BGRA8UNORM_STORAGE, // TODO: test for webgpu-mobile
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        device.on_uncaptured_error(Box::new(|error| {
            panic!("Aborting due to an error: {}", error);
        }));

        return Gpu { device, queue };
    }
}
