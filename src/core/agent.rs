
use anyhow::Result;

pub struct Agent {
    name: String,
}

impl Agent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }

    pub async fn process(&self, input: &str) -> Result<String> {
        let input = input.trim();

        if input.is_empty() {
            return Ok("I didn't hear anything.".to_string());
        }

        let response = match input.to_lowercase().as_str() {
            "hello" | "hi" | "hey" => {
                format!("Hello. I'm {}.", self.name)
            }

            "who are you" | "what are you" => {
                format!(
                    "I'm {}, a local autonomous AI assistant.",
                    self.name
                )
            }

            "status" => {
                "All core systems are operational.".to_string()
            }

            "exit" | "quit" => {
                "Goodbye.".to_string()
            }

            _ => {
                format!(
                    "I received: \"{}\". My reasoning engine isn't connected yet.",
                    input
                )
            }
        };

        Ok(response)
    }
}