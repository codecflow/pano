mod builder;
mod cli;
mod commands;
mod error;
mod uds;

use clap::Parser;
use std::sync::mpsc::channel;
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use wry::WebViewBuilder;

use builder::{AppWindow, WindowFactory};
use cli::Options;
use commands::Command;
use uds::UDSListener;

fn main() -> error::Result<()> {
    let options = Options::parse();
    let (tx, rx) = channel::<Command>();

    let mut uds = UDSListener::new(options.socket.clone());
    uds.start(tx)?;

    let event_loop = EventLoop::new();
    let window = WindowFactory::create_window_builder()
        .with_always_on_top(true)
        .with_transparent(true)
        .with_position(LogicalPosition::new(options.x, options.y))
        .with_inner_size(LogicalSize::new(options.width, options.height))
        .with_visible(true)
        .build(&event_loop)
        .map_err(|e| error::AppError::Window(e.to_string()))?;

    let builder = WebViewBuilder::new()
        .with_url(&options.url)
        .with_transparent(true);

    if options.gpu {
        eprintln!("Warning: GPU acceleration is not supported in wry 0.53.3");
    }

    let webview = WindowFactory::create_webview(builder, &window)?;
    let app_window = AppWindow::new(window, webview);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(command) = rx.try_recv() {
            match command {
                Command::UpdateUrl(url) => app_window.update_url(&url),
                Command::Resize(width, height) => app_window.resize(width, height),
                Command::Move(x, y) => app_window.move_to(x, y),
            }
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
