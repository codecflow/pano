use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::macos::WindowBuilderExtMacOS,
    window::{Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

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
fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = get_window_builder()
        .with_always_on_top(true)
        .with_transparent(true)
        .with_position(LogicalPosition::new(0, 0))
        .with_inner_size(LogicalSize::new(200, 200))
        .with_visible(true)
        .build(&event_loop)
        .unwrap();

    let builder = WebViewBuilder::new()
        .with_url("https://example.com/")
        .with_transparent(true);

    let _webview = get_webview(builder, &window)?;

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
