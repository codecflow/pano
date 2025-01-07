#[derive(Debug)]
pub enum Command {
    UpdateUrl(String),
    Resize(u32, u32),
    Move(i32, i32),
}