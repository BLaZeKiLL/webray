pub struct Metrics {
    start: std::time::Instant,
    device_acquisition: std::time::Duration,
    scene_upload: std::time::Duration,
    kernel_initialization: std::time::Duration,
    rendering: std::time::Duration,
    output_write: std::time::Duration,
    total: std::time::Duration,
}

impl Metrics {
    pub fn new() -> Self {
        return Metrics {
            start: std::time::Instant::now(),
            device_acquisition: std::time::Duration::ZERO,
            scene_upload: std::time::Duration::ZERO,
            kernel_initialization: std::time::Duration::ZERO,
            rendering: std::time::Duration::ZERO,
            output_write: std::time::Duration::ZERO,
            total: std::time::Duration::ZERO,
        };
    }

    pub fn start(&mut self) {
        self.start = std::time::Instant::now();
    }

    pub fn capture_device_acquisition(&mut self) {
        self.device_acquisition = self.start.elapsed();
    }

    pub fn capture_scene_upload(&mut self) {
        self.scene_upload = self.start.elapsed() - self.device_acquisition;
    }

    pub fn capture_kernel_initialization(&mut self) {
        self.kernel_initialization = self.start.elapsed() - self.scene_upload;
    }

    pub fn capture_rendering(&mut self) {
        self.rendering = self.start.elapsed() - self.kernel_initialization;
    }

    pub fn capture_output_write(&mut self) {
        self.output_write = self.start.elapsed() - self.rendering;
    }

    pub fn capture_total(&mut self) {
        self.total = self.start.elapsed();
    }

    pub fn log(&self) {
        log::info!("===== WebRay Metrics =====");
        log::info!(
            "Device Acquisition: {} secs",
            self.device_acquisition.as_secs_f64()
        );
        log::info!("Scene Upload: {} secs", self.scene_upload.as_secs_f64());
        log::info!(
            "Kernel Initialization: {} secs",
            self.kernel_initialization.as_secs_f64()
        );
        log::info!("Rendering: {} secs", self.rendering.as_secs_f64());
        log::info!("Output write: {} secs", self.output_write.as_secs_f64());
        log::info!("Total: {} secs", self.total.as_secs_f64());
        log::info!("==========================");
    }
}
