use std::{
    io::{BufRead, BufReader},
    os::unix::net::UnixListener,
    path::PathBuf,
    sync::mpsc::Sender,
    thread,
};

use crate::{
    commands::Command,
    error::{AppError, Result},
};

pub struct UDSListener {
    socket_path: PathBuf,
}

impl UDSListener {
    pub fn new(socket_path: PathBuf) -> Self {
        Self { socket_path }
    }

    pub fn start(&self, tx: Sender<Command>) -> Result<()> {
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)
                .map_err(|e| AppError::IpcError(e.to_string()))?;
        }

        let listener =
            UnixListener::bind(&self.socket_path).map_err(|e| AppError::IpcError(e.to_string()))?;

        let socket_path = self.socket_path.clone();
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let reader = BufReader::new(stream);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                Self::handle_command(&line, &tx);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error accepting connection: {}", e),
                }
            }
        });

        Ok(())
    }

    fn handle_command(line: &str, tx: &Sender<Command>) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.get(0).map(|s| *s) {
            Some("url") => {
                if let Some(url) = parts.get(1) {
                    tx.send(Command::UpdateUrl(url.to_string())).unwrap();
                }
            }
            Some("resize") => {
                if let (Some(width), Some(height)) = (parts.get(1), parts.get(2)) {
                    if let (Ok(w), Ok(h)) = (width.parse(), height.parse()) {
                        tx.send(Command::Resize(w, h)).unwrap();
                    }
                }
            }
            Some("move") => {
                if let (Some(x), Some(y)) = (parts.get(1), parts.get(2)) {
                    if let (Ok(x), Ok(y)) = (x.parse(), y.parse()) {
                        tx.send(Command::Move(x, y)).unwrap();
                    }
                }
            }
            _ => {}
        }
    }
}
