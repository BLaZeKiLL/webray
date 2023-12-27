pub struct KernelConfig {
    pub width: u32,
    pub height: u32,
    pub size: u64
}

impl KernelConfig {
    pub fn new(width: u32, height: u32) -> Self {
        return KernelConfig {
            width,
            height,
            size: (width * height * 4) as u64
        };
    }
}