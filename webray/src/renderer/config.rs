#[derive(Debug, encase::ShaderType)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    samples: u32,
    bounces: u32,
}

#[derive(Debug, encase::ShaderType)]
pub struct Camera {
    center: glam::Vec3,
    dof_angle: f32,
    dof_disk_u: glam::Vec3,
    dof_disk_v: glam::Vec3
}

#[derive(Debug, encase::ShaderType)]
pub struct Viewport {
    width: f32,
    height: f32,
    u: glam::Vec3, // local horizontal vector
    v: glam::Vec3, // local inverted vertical vector
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

pub enum TileSize {
    Full,
    Square(u32)
}

pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub bounces: u32,
    pub tile_size: TileSize
}

pub struct CameraConfig {
    pub look_from: glam::Vec3,
    pub look_at: glam::Vec3,
    pub v_up: glam::Vec3,
    pub v_fov: f32,
    pub dof_angle: f32,
    pub dof_distance: f32
}

impl KernelConfig {
    /// A lot of camera calculations
    pub fn new(render_config: RenderConfig, camera_config: CameraConfig) -> Self {
        // Determine viewport dimensions.
        let h = (camera_config.v_fov.to_radians() / 2.0).tan(); // 90 deg this equation = 1.0
        let viewport_height = 2.0 * h * camera_config.dof_distance;
        let viewport_width =
            viewport_height * (render_config.width as f32 / render_config.height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (camera_config.look_from - camera_config.look_at).normalize();
        let u = camera_config.v_up.cross(w).normalize();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors to the next pixel.
        let delta_u = viewport_u / render_config.width as f32;
        let delta_v = viewport_v / render_config.height as f32;

        // Calculate the location of the upper left pixel.
        let upper_left = camera_config.look_from
            - (camera_config.dof_distance * w) 
            - (viewport_u / 2.0) 
            - (viewport_v / 2.0);
        let pixel_zero_loc = upper_left + 0.5 * (delta_u + delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = camera_config.dof_distance * (camera_config.dof_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let image = Image {
            width: render_config.width,
            height: render_config.height,
            samples: render_config.samples,
            bounces: render_config.bounces,
        };

        let camera = Camera {
            center: camera_config.look_from,
            dof_angle: camera_config.dof_angle,
            dof_disk_u: defocus_disk_u,
            dof_disk_v: defocus_disk_v
        };

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
            pixel_zero_loc,
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
