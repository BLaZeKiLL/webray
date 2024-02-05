use rand::{rngs::ThreadRng, Rng};

pub fn random_color(rng: &mut ThreadRng) -> glam::Vec3 {
    let r: f32 = rng.gen();
    let g: f32 = rng.gen();
    let b: f32 = rng.gen();

    return glam::vec3(r, g, b);
}

pub fn random_color_range(rng: &mut ThreadRng, min: f32, max: f32) -> glam::Vec3 {
    let r: f32 = rng.gen_range(min..max);
    let g: f32 = rng.gen_range(min..max);
    let b: f32 = rng.gen_range(min..max);

    return glam::vec3(r, g, b);
}

pub fn hex_to_rgb(hex: &str) -> Option<glam::Vec3> {
    // Check if the hex string starts with '#' and remove it
    let hex = if hex.starts_with('#') { &hex[1..] } else { hex };

    // Ensure that the hex string is of valid length (either 3 or 6)
    if hex.len() != 3 && hex.len() != 6 {
        return None;
    }

    // Parse the hex string into a u32
    let hex_int = u32::from_str_radix(hex, 16).ok()?;

    // Extract the color components
    let r = if hex.len() == 3 {
        ((hex_int >> 8) & 0xF) as u8 * 17
    } else {
        (hex_int >> 16) as u8
    };
    let g = if hex.len() == 3 {
        ((hex_int >> 4) & 0xF) as u8 * 17
    } else {
        ((hex_int >> 8) & 0xFF) as u8
    };
    let b = if hex.len() == 3 {
        (hex_int & 0xF) as u8 * 17
    } else {
        (hex_int & 0xFF) as u8
    };

    return Some(glam::vec3(r.into(), g.into(), b.into()) / 255.0);
}
