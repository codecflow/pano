use clap::{Arg, Command};
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::x11::WindowBuilderExtX11,
    window::WindowBuilder,
};
use wry::webview::WebViewBuilder;

fn main() -> wry::Result<()> {
    let matches = Command::new("Pano")
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .default_value("800")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("height")
                .short('h')
                .long("height")
                .default_value("600")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("x")
                .short('x')
                .long("x")
                .default_value("100")
                .value_parser(clap::value_parser!(i32)),
        )
        .arg(
            Arg::new("y")
                .short('y')
                .long("y")
                .default_value("100")
                .value_parser(clap::value_parser!(i32)),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .default_value("data:text/html,"),
        )
        .get_matches();

    let width = *matches.get_one::<u32>("width").unwrap();
    let height = *matches.get_one::<u32>("height").unwrap();
    let x = *matches.get_one::<i32>("x").unwrap();
    let y = *matches.get_one::<i32>("y").unwrap();
    let url = matches.get_one::<String>("url").unwrap();

    // Create the Winit event loop
    let event_loop = EventLoop::new();

    // Create a window with transparency and X11-specific settings
    let window = WindowBuilder::new()
        .with_title("Overlay App")
        .with_inner_size(LogicalSize::new(width, height))
        .with_position(LogicalPosition::new(x, y))
        .with_transparent(true)
        .build(&event_loop)
        .unwrap();

    // Create a Wry WebView with a transparent background
    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_transparent(true)
        .with_url(url)
        .unwrap()
        .build()
        .unwrap();

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
