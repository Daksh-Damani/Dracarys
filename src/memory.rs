use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const MEMORY_FILE: &str = "memory/conversation.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct MemoryMessage {
    pub role: String,
    pub content: String,
}

pub fn load() -> Vec<MemoryMessage> {
    if !Path::new(MEMORY_FILE).exists() {
        return Vec::new();
    }

    match fs::read_to_string(MEMORY_FILE) {
        Ok(data) => {
            if data.trim().is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&data).unwrap_or_default()
            }
        }
        Err(_) => Vec::new(),
    }
}

pub fn save(messages: &[MemoryMessage]) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(messages)?;
    fs::write(MEMORY_FILE, data)?;
    Ok(())
}

pub fn clear() -> Result<(), Box<dyn std::error::Error>> {
    save(&[])
}