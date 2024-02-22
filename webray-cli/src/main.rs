fn main() {
    webray::initialize_kernel();
    webray::render(include_str!("../../src/data/demo_02.scene.json").to_string());
}