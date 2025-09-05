use crate::error::{AppError, Result};
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    window::{Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

pub struct AppWindow {
    pub window: Window,
    pub webview: WebView,
}

impl AppWindow {
    pub fn new(window: Window, webview: WebView) -> Self {
        Self { window, webview }
    }

    pub fn update_url(&self, url: &str) {
        if let Err(e) = self.webview.load_url(url) {
            eprintln!("Failed to load URL '{}': {}", url, e);
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.window.set_inner_size(LogicalSize::new(width, height));
    }

    pub fn move_to(&self, x: i32, y: i32) {
        self.window.set_outer_position(LogicalPosition::new(x, y));
    }
}

pub struct WindowFactory;

impl WindowFactory {
    #[cfg(target_os = "macos")]
    pub fn create_window_builder() -> WindowBuilder {
        use tao::platform::macos::WindowBuilderExtMacOS;
        WindowBuilder::new().with_titlebar_hidden(true)
    }

    #[cfg(not(target_os = "macos"))]
    pub fn create_window_builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    pub fn create_webview(builder: WebViewBuilder, window: &Window) -> Result<WebView> {
        builder
            .build(window)
            .map_err(|e| AppError::WebView(e.to_string()))
    }

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    pub fn create_webview(builder: WebViewBuilder, window: &Window) -> Result<WebView> {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window
            .default_vbox()
            .ok_or_else(|| AppError::WebView("Failed to get default vbox".to_string()))?;
        builder
            .build_gtk(vbox)
            .map_err(|e| AppError::WebView(e.to_string()))
    }
}
