#[derive(Debug, encase::ShaderType)]
pub struct Image {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, encase::ShaderType)]
pub struct Camera {
    /// Camera center is the eye point
    pub center: glam::Vec3,
    /// Distance from camera center and viewport center
    pub focal_length: f32,
    pub samples: u32,
    pub bounces: u32,
}

#[derive(Debug, encase::ShaderType)]
pub struct Viewport {
    width: f32,
    height: f32,
    /// horizontal vector
    u: glam::Vec3,
    /// inverted vertical vector
    v: glam::Vec3,
    delta_u: glam::Vec3,
    delta_v: glam::Vec3,
    upper_left: glam::Vec3,
}

#[derive(Debug, encase::ShaderType)]
pub struct KernelConfig {
    pub image: Image,
    pub camera: Camera,
    pub viewport: Viewport,
    pixel_zero_loc: glam::Vec3,
}

impl KernelConfig {
    pub fn new(width: u32, height: u32) -> Self {
        let image = Image { width, height };

        let camera = Camera {
            center: glam::Vec3::ZERO,
            focal_length: 1.0,
            samples: 128,
            bounces: 64,
        };

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image.width as f32 / image.height as f32);

        let viewport_u = glam::vec3(viewport_width, 0.0, 0.0);
        let viewport_v = glam::vec3(0.0, -viewport_height, 0.0);

        let delta_u = viewport_u / image.width as f32;
        let delta_v = viewport_v / image.height as f32;

        let upper_left = camera.center
            - glam::vec3(0.0, 0.0, camera.focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let viewport = Viewport {
            width: viewport_width,
            height: viewport_height,
            u: viewport_u,
            v: viewport_v,
            delta_u,
            delta_v,
            upper_left,
        };

        return KernelConfig {
            image,
            camera,
            viewport,
            pixel_zero_loc: upper_left + 0.5 * (delta_u + delta_v),
        };
    }

    pub fn result_size(&self) -> u64 {
        return (self.image.width * self.image.height * 4) as u64;
    }

    pub fn as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self).unwrap();
        return Ok(buffer.into_inner());
    }
}
