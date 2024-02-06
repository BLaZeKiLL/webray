use wasm_bindgen::JsCast;

pub fn output_image(image_data: Vec<u8>, dimensions: glam::UVec2) {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = if let Some(found_canvas) = document.get_element_by_id("staging-canvas") {
        match found_canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
            Ok(canvas_as_canvas) => canvas_as_canvas,
            Err(e) => {
                e.remove();
                create_staging_canvas(&document)
            }
        }
    } else {
        create_staging_canvas(&document)
    };

    let image_dimension_strings = (dimensions.x.to_string(), dimensions.y.to_string());

    canvas
        .set_attribute("width", image_dimension_strings.0.as_str())
        .unwrap();
    canvas
        .set_attribute("height", image_dimension_strings.1.as_str())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let image_data = web_sys::ImageData::new_with_u8_clamped_array(
        wasm_bindgen::Clamped(&image_data),
        dimensions.x,
    )
    .unwrap();

    context.put_image_data(&image_data, 0.0, 0.0).unwrap();

    let image_element =
        if let Some(found_image_element) = document.get_element_by_id("output-image-target") {
            match found_image_element.dyn_into::<web_sys::HtmlImageElement>() {
                Ok(e) => e,
                Err(e) => {
                    e.remove();
                    create_output_image_element(&document)
                }
            }
        } else {
            create_output_image_element(&document)
        };

    let data_url = canvas.to_data_url().unwrap();
    image_element.set_src(&data_url);

    log::info!("Image displayed");
}

fn create_staging_canvas(document: &web_sys::Document) -> web_sys::HtmlCanvasElement {
    let body = document.body().expect("Failed to get document body.");
    let new_canvas = document
        .create_element("canvas")
        .expect("Failed to create staging canvas.")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    // We don't want to show the canvas, we just want it to exist in the background.
    new_canvas.set_attribute("hidden", "true").unwrap();
    new_canvas.set_attribute("background-color", "red").unwrap();
    body.append_child(&new_canvas).unwrap();

    return new_canvas;
}

fn create_output_image_element(document: &web_sys::Document) -> web_sys::HtmlImageElement {
    let body = document.body().expect("Failed to get document body.");
    let new_image = document
        .create_element("img")
        .expect("Failed to create output image element.")
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();

    new_image.set_id("output-image-target");

    body.append_child(&new_image)
        .expect("Failed to append output image target to document body.");

    return new_image;
}
