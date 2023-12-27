#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use log::{info, error};

use winit::error::EventLoopError;
use winit::event::{KeyEvent, ElementState};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::{event::WindowEvent, event_loop::EventLoopWindowTarget};

use crate::{
    app::App,
    core::{gpu::Gpu, window::Window},
};

mod app;
mod core;

pub async fn run() -> Result<(), EventLoopError> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    let window = Window::new();

    info!("Window initialized");

    let gpu = Gpu::new(&window).await;

    info!("WebGPU initialized");

    let app = App::new(gpu);

    return start(window, app);
}

fn start(window: Window, mut app: App) -> Result<(), EventLoopError> {
    app.start();

    return window
        .event_loop
        .run(move |event, target| match event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.handle.id() => {
                // if this window
                handle_window_event(&mut app, event, target);
            }
            winit::event::Event::AboutToWait => {
                //window.handle.request_redraw(); // re-render each frame
            }
            _ => {}
        });
}

fn handle_window_event(app: &mut App, event: &WindowEvent, target: &EventLoopWindowTarget<()>) {
    if app.input(event) {
        // if app has processed the event
        return;
    }

    match event {
        winit::event::WindowEvent::CloseRequested
        | winit::event::WindowEvent::KeyboardInput {
            event: KeyEvent {
                physical_key: PhysicalKey::Code(KeyCode::Escape),
                state: ElementState::Pressed,
                ..
            },
            ..
        } => {
            app.destroy();

            target.exit();
        }

        winit::event::WindowEvent::Resized(physical_size) => {
            app.resize(*physical_size);
        }

        winit::event::WindowEvent::RedrawRequested => {
            app.update();

            match app.render() {
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::OutOfMemory) => {
                    target.exit();
                }
                Err(e) => error!("{:?}", e),
            }
        }

        _ => {}
    }
}
