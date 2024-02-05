use core::fmt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WScene {
    pub objects: Vec<WObject>,
    pub materials: Vec<WMaterial>,
    pub camera: WCamera,
    pub render_settings: WRenderSettings
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WObject {
    pub id: u32,
    pub name: String,
    pub material_id: usize,

    #[serde(rename = "type")]
    pub obj_type: WObjectType
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum WObjectType {
    #[serde(rename = "d_sphere")]
    Sphere { position: glam::Vec3, radius: f32 }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WMaterial {
    pub id: u32,
    pub name: String,

    #[serde(rename = "type")]
    pub mat_type: WMaterialType
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum WMaterialType {
    #[serde(rename = "d_mat_diffuse")]
    Diffuse { color: String },

    #[serde(rename = "d_mat_metal")]
    Metal { color: String, roughness: f32 },

    #[serde(rename = "d_mat_dielectric")]
    Dielectric { ior: f32 }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WCamera {
    pub look_from: glam::Vec3,
    pub look_at: glam::Vec3,
    pub v_up: glam::Vec3,
    pub v_fov: f32,
    pub dof_angle: f32,
    pub dof_distance: f32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WRenderSettings {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub bounces: u32,
    pub tile_size: WTileSize
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum WTileSize {
    #[serde(rename = "d_tile_size_full")]
    Full,

    #[serde(rename = "d_tile_size")]
    Tile { size: u32 }
}

impl fmt::Display for WScene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Objects:").unwrap();

        for item in &self.objects {
            write!(f, "\n\tObject: {}", item).unwrap();
        }

        write!(f, "\nMaterials:").unwrap();

        for item in &self.materials {
            write!(f, "\n\tMaterial: {}", item).unwrap();
        }

        return write!(f, "\nCamera:\n\t{}\nRenderSettings:\n\t{}", self.camera, self.render_settings);
    }
}

impl fmt::Display for WObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "\n\t\tID: {}\n\t\tname: {}\n\t\tmaterial_id: {}\n\t\ttype: {}", self.id, self.name, self.material_id, self.obj_type);
    }
}

impl fmt::Display for WObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            WObjectType::Sphere { position, radius } => write!(f, "SPHERE(position: {}, radius: {})", position, radius),
        }
    }
}

impl fmt::Display for WMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "\n\t\tID: {}\n\t\tname: {}\n\t\ttype: {}", self.id, self.name, self.mat_type);
    }
}

impl fmt::Display for WMaterialType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            WMaterialType::Diffuse { color } => write!(f, "DIFFUSE(color: {})", color),
            WMaterialType::Metal { color, roughness } => write!(f, "METAL(color: {}, roughness: {})", color, roughness),
            WMaterialType::Dielectric { ior } => write!(f, "DIELECTRIC(ior: {})", ior),
        }
    }
}

impl fmt::Display for WCamera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "look_from: {}\n\tlook_at: {}\n\tv_up: {}\n\tv_fov: {}\n\tdof_angle: {}\n\tdof_distance: {}", self.look_from, self.look_at, self.v_up, self.v_fov, self.dof_angle, self.dof_distance);
    }
}

impl fmt::Display for WRenderSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "width: {}\n\theight: {}\n\tsamples: {}\n\tbounces: {}\n\ttile size: {}", self.width, self.height, self.samples, self.bounces, self.tile_size);
    }
}

impl fmt::Display for WTileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            WTileSize::Full => write!(f, "FULL()"),
            WTileSize::Tile { size } => write!(f, "TILE(size: {})", size),
        }
    }
}