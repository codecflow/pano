use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Window(String),
    WebView(String),
    Ipc(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Window(msg) => write!(f, "Window error: {}", msg),
            AppError::WebView(msg) => write!(f, "WebView error: {}", msg),
            AppError::Ipc(msg) => write!(f, "IPC error: {}", msg),
        }
    }
}

impl Error for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;
