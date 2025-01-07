use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    WindowError(String),
    WebViewError(String),
    IpcError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::WindowError(msg) => write!(f, "Window error: {}", msg),
            AppError::WebViewError(msg) => write!(f, "WebView error: {}", msg),
            AppError::IpcError(msg) => write!(f, "IPC error: {}", msg),
        }
    }
}

impl Error for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;
