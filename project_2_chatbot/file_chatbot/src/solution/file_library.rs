use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!


pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    let bytes = match fs::read(filename) {
        Ok(data) => data,
        Err(_) => return None,
    };
    
    LlamaChatSession::from_bytes(&bytes).ok()
}

/// Saves a LlamaChatSession to a file.
pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = session.to_bytes()?;
    fs::write(filename, bytes)?;
    Ok(())
}
