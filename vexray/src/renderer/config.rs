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

pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub bounces: u32,
}

pub struct CameraConfig {
    pub look_from: glam::Vec3,
    pub look_at: glam::Vec3,
    pub vup: glam::Vec3,
    pub vfov: f32,
}

impl KernelConfig {
    /// A lot of camera calculations
    pub fn new(render_config: RenderConfig, camera_config: CameraConfig) -> Self {
        let image = Image {
            width: render_config.width,
            height: render_config.height,
        };

        let camera = Camera {
            center: camera_config.look_from,
            focal_length: (camera_config.look_from - camera_config.look_at).length(),
            samples: render_config.samples,
            bounces: render_config.bounces,
        };

        let h = (camera_config.vfov.to_radians() / 2.0).tan(); // 90 deg this equation = 1.0
        let viewport_height = 2.0 * h * camera.focal_length;
        let viewport_width = viewport_height * (image.width as f32 / image.height as f32);

        let w = (camera_config.look_from - camera_config.look_at).normalize();
        let u = camera_config.vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let delta_u = viewport_u / image.width as f32;
        let delta_v = viewport_v / image.height as f32;

        let upper_left =
            camera.center - (camera.focal_length * w) - (viewport_u / 2.0) - (viewport_v / 2.0);

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
