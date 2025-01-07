use std::path::PathBuf;

use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(name = "pano", version, about, disable_help_flag = true)]
struct Options {
    #[arg(short, long, default_value = "", help = "URL to open")]
    url: String,

    #[arg(short, long, default_value_t = 800, help = "Width of the window")]
    width: u32,

    #[arg(short, long, default_value_t = 600, help = "Height of the window")]
    height: u32,

    #[arg(short, default_value_t = 0, help = "X position of the window")]
    x: i32,

    #[arg(short, default_value_t = 0, help = "Y position of the window")]
    y: i32,

    #[arg(long, default_value_t = false, help = "Enable GPU acceleration")]
    gpu: bool,

    #[arg(long, default_value = "/tmp/pano", help = "Path to the socket file")]
    socket: PathBuf,

    #[arg(long, action = ArgAction::Help, help = "Print help information")]
    help: Option<u8>,
}

#[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
))]
fn get_webview(builder: WebViewBuilder, window: &Window) -> wry::Result<WebView> {
    builder.build(window)
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
)))]
fn get_webview(builder: WebViewBuilder, window: &Window) -> wry::Result<WebView> {
    use tao::platform::unix::WindowExtUnix;
    use wry::WebViewBuilderExtUnix;
    let vbox = window.default_vbox().unwrap();
    builder.build_gtk(vbox)
}

#[cfg(target_os = "macos")]
fn get_window_builder() -> WindowBuilder {
    use tao::platform::macos::WindowBuilderExtMacOS;
    WindowBuilder::new().with_titlebar_hidden(true)
}

fn main() {
    let options = Options::parse();

    let event_loop = EventLoop::new();
    let window = get_window_builder()
        .with_always_on_top(true)
        .with_transparent(true)
        .with_position(LogicalPosition::new(options.x, options.y))
        .with_inner_size(LogicalSize::new(options.width, options.height))
        .with_visible(true)
        .build(&event_loop)
        .unwrap();

    let builder = WebViewBuilder::new()
        .with_url(options.url)
        .with_transparent(true);

    let _webview = get_webview(builder, &window).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
