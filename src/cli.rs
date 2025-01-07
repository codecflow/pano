use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "pano", version, about, disable_help_flag = true)]
pub struct Options {
    #[arg(short, long, default_value = "", help = "URL to open")]
    pub url: String,

    #[arg(short, long, default_value_t = 800, help = "Width of the window")]
    pub width: u32,

    #[arg(short, long, default_value_t = 600, help = "Height of the window")]
    pub height: u32,

    #[arg(short, default_value_t = 0, help = "X position of the window")]
    pub x: i32,

    #[arg(short, default_value_t = 0, help = "Y position of the window")]
    pub y: i32,

    #[arg(long, default_value_t = false, help = "Enable GPU acceleration")]
    pub gpu: bool,

    #[arg(long, default_value = "/tmp/pano", help = "Path to the socket file")]
    pub socket: PathBuf,

    #[arg(long, action = ArgAction::Help, help = "Print help information")]
    pub help: Option<u8>,
}
