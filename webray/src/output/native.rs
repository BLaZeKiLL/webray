pub fn output_image(image_data: Vec<u8>, dimensions: glam::UVec2, path: &str) {
    match image::save_buffer(
        path,
        &image_data,
        dimensions.x,
        dimensions.y,
        image::ColorType::Rgba8,
    ) {
        Ok(_) => log::info!("Output saved at path: {}", path),
        Err(e) => log::error!("{:?}", e),
    }
}
