// Starting of my legendary tool Dracarys [name inspired by GOT have't watched it yet but name is good and i cant think of any good name except boring delulu so here we are ]

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::fs;
mod memory;

const SERVER_URL: &str = "http://127.0.0.1:8080";
const SYSTEM_PROMPT_PATH: &str = "config/system.txt";

#[derive(Serialize)]
struct ChatRequest {
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: i32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("DRACARYS");
    println!("Local AI runtime");
    println!("Backend: llama.cpp");
    println!("Model: Qwen3-4B");
    println!();

    let system_prompt = fs::read_to_string(SYSTEM_PROMPT_PATH)
        .map_err(|e| {
            format!(
                "Failed to load system prompt '{}': {}",
                SYSTEM_PROMPT_PATH, e
            )
        })?;

    let client = Client::new();

    let health = client
        .get(format!("{SERVER_URL}/health"))
        .send()
        .await?;

    if !health.status().is_success() {
        eprintln!("llama-server is not ready.");
        eprintln!("Start llama-server first.");
        return Ok(());
    }

    println!("Backend: online");
    println!("Type /exit to quit.");
    println!();
    let mut history = Vec::<Message>::new();

    history.push(Message {
      role: "system".to_string(),
      content: system_prompt,
    });

    let saved_memory = memory::load();

    for message in saved_memory {
        history.push(Message {
            role: message.role,
            content: message.content,
        });
    }

    loop {
        print!("You > ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("/exit") {
            break;
        }

        if input.eq_ignore_ascii_case("/memory") {
    let saved = memory::load();

    if saved.is_empty() {
        println!();
        println!("Dracarys > No saved memories.");
        println!();
    } else {
        println!();
        println!("Dracarys memory:");

        for message in &saved {
            println!("{} > {}", message.role, message.content);
        }

        println!();
    }

    continue;
}

if input.eq_ignore_ascii_case("/forget") {
    memory::clear()?;
    history.retain(|message| message.role == "system");

    println!();
    println!("Dracarys > Persistent memory cleared.");
    println!();

    continue;
}

if input.eq_ignore_ascii_case("/clear") {
    history.retain(|message| message.role == "system");

    println!();
    println!("Dracarys > Current conversation cleared.");
    println!();

    continue;
}

if let Some(text) = input.strip_prefix("/remember ") {
    let text = text.trim();

    if text.is_empty() {
        println!();
        println!("Dracarys > Tell me what you want me to remember.");
        println!();
        continue;
    }

    history.push(Message {
        role: "user".to_string(),
        content: format!("Remember this: {text}"),
    });

    let memory_messages: Vec<memory::MemoryMessage> = memory::load();

    let mut updated_memory = memory_messages;

    updated_memory.push(memory::MemoryMessage {
        role: "memory".to_string(),
        content: text.to_string(),
    });

    memory::save(&updated_memory)?;

    history.pop();

    println!();
    println!("Dracarys > I'll remember that.");
    println!();

    continue;
}

        history.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        let request = ChatRequest {
            messages: history.clone(),
            temperature: 0.7,
            max_tokens: 512,
        };

        let response = client
            .post(format!("{SERVER_URL}/v1/chat/completions"))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            eprintln!("Server returned: {}", response.status());
            continue;
        }

        let result: ChatResponse = response.json().await?;

        let Some(choice) = result.choices.first() else {
            eprintln!("No response from model.");
            continue;
        };

        let answer = &choice.message.content;

        println!();
        println!("Dracarys > {answer}");
        println!();

        history.push(choice.message.clone());

        let memory_messages: Vec<memory::MemoryMessage> = history
    .iter()
    .filter(|message| message.role != "system")
    .map(|message| memory::MemoryMessage {
        role: message.role.clone(),
        content: message.content.clone(),
    })
    .collect();

        if let Err(error) = memory::save(&memory_messages) {
        eprintln!("Warning: failed to save memory: {error}");
    }
    }

    println!("Dracarys shutting down.");

    Ok(())
}