fn main() {
    webray::initialize_kernel();

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            log::error!("Platform not supported");
        } else {
            webray::render();
        }
    }
}