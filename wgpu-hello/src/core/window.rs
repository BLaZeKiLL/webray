pub struct Window {
    pub handle: winit::window::Window,
    pub event_loop: winit::event_loop::EventLoop<()>,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let handle = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        return Window {
            handle,
            event_loop,
        };
    }
}