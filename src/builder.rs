use crate::cli::Options;
use crate::error::{AppError, Result};
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    window::{Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

#[cfg(feature = "x11")]
use x11_dl::xlib::{self, Display, Visual, XVisualInfo};

pub struct AppWindow {
    pub window: Window,
    pub webview: WebView,
}

impl AppWindow {
    pub fn new(_options: &Options, window: Window, webview: WebView) -> Self {
        Self { window, webview }
    }

    pub fn update_url(&self, url: &str) {
        let _ = self.webview.load_url(url);
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.window.set_inner_size(LogicalSize::new(width, height));
    }

    pub fn move_to(&self, x: i32, y: i32) {
        self.window.set_ime_position(LogicalPosition::new(x, y));
    }
}

#[cfg(feature = "x11")]
pub fn configure_x11_transparency() -> Result<()> {
    use std::ptr;
    
    unsafe {
        let xlib = xlib::Xlib::open().map_err(|e| AppError::WindowError(format!("Failed to open X11: {}", e)))?;
        let display = (xlib.XOpenDisplay)(ptr::null());
        if display.is_null() {
            return Err(AppError::WindowError("Failed to open X11 display".to_string()));
        }

        let screen = (xlib.XDefaultScreen)(display);
        let mut visual_info = XVisualInfo {
            visual: ptr::null_mut(),
            visualid: 0,
            screen,
            depth: 32,
            class: 4, // TrueColor
            red_mask: 0,
            green_mask: 0,
            blue_mask: 0,
            colormap_size: 0,
            bits_per_rgb: 0,
        };

        let mut nitems = 0;
        let visual_list = (xlib.XGetVisualInfo)(
            display,
            xlib::VisualScreenMask | xlib::VisualDepthMask | xlib::VisualClassMask,
            &mut visual_info,
            &mut nitems,
        );

        if !visual_list.is_null() && nitems > 0 {
            // Found 32-bit visual, transparency should work
            (xlib.XFree)(visual_list as *mut _);
        }

        (xlib.XCloseDisplay)(display);
    }
    Ok(())
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
            .map_err(|e| AppError::WebViewError(e.to_string()))
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
        let vbox = window.default_vbox().unwrap();
        builder
            .build_gtk(vbox)
            .map_err(|e| AppError::WebViewError(e.to_string()))
    }
}
