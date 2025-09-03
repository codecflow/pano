use std::{
    io::{BufRead, BufReader},
    os::unix::net::UnixListener,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::{
    commands::Command,
    error::{AppError, Result},
};

pub struct UDSListener {
    socket_path: PathBuf,
    thread_handle: Option<JoinHandle<()>>,
    shutdown: Arc<AtomicBool>,
}

impl UDSListener {
    pub fn new(socket_path: PathBuf) -> Self {
        Self { 
            socket_path,
            thread_handle: None,
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self, tx: Sender<Command>) -> Result<()> {
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)
                .map_err(|e| AppError::Ipc(e.to_string()))?;
        }

        let listener =
            UnixListener::bind(&self.socket_path).map_err(|e| AppError::Ipc(e.to_string()))?;
        
        listener.set_nonblocking(true)
            .map_err(|e| AppError::Ipc(e.to_string()))?;

        let shutdown = self.shutdown.clone();
        let handle = thread::spawn(move || {
            while !shutdown.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let reader = BufReader::new(stream);
                        for line in reader.lines() {
                            if shutdown.load(Ordering::Relaxed) {
                                break;
                            }
                            match line {
                                Ok(line) => Self::handle_command(&line, &tx),
                                Err(e) => {
                                    eprintln!("Error reading line: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(e) => eprintln!("Error accepting connection: {}", e),
                }
            }
        });

        self.thread_handle = Some(handle);
        Ok(())
    }

    fn handle_command(line: &str, tx: &Sender<Command>) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        let result = match parts.as_slice() {
            ["url", url] => tx.send(Command::UpdateUrl(url.to_string())),
            ["resize", width, height] => {
                match (width.parse::<u32>(), height.parse::<u32>()) {
                    (Ok(w), Ok(h)) => tx.send(Command::Resize(w, h)),
                    _ => return,
                }
            }
            ["move", x, y] => {
                match (x.parse::<i32>(), y.parse::<i32>()) {
                    (Ok(x), Ok(y)) => tx.send(Command::Move(x, y)),
                    _ => return,
                }
            }
            _ => return,
        };
        
        if let Err(e) = result {
            eprintln!("Failed to send command: {}", e);
        }
    }
}

impl Drop for UDSListener {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
        
        if self.socket_path.exists()
            && let Err(e) = std::fs::remove_file(&self.socket_path) {
                eprintln!("Failed to remove socket file: {}", e);
            }
    }
}
