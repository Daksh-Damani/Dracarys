// Starting of my legendary tool Dracarys [name inspired by GOT have't watched it yet but name is good and i cant think of any good name except boring delulu so here we are ]

mod core;
mod llm;
mod tools;

use anyhow::Result;
use core::agent::Agent;
use std::io::{self, Write};
use llm::{DracarysInference, InferenceEngine, GenerationRequest};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("=================================");
    println!("        DRACARYS v0.1.0");
    println!("     Local Lovable AI Thingy     ");
    println!("=================================");
    println!();

    println!("Core:   READY");
    println!("Agent:  READY");
    println!("LLM:    NOT CONNECTED");
    println!("Tools:  READY");
    println!();

    let agent = Agent::new("DRACARYS");
    let mut inference = InferenceEngine::new("DRACARYS");

    inference.load()?;

    println!("Inference: READY");
    println!("Model: {}", inference.model_name());
    println!();

    println!("DRACARYS: Hello. I'm online.");
    println!();

    let test_response = inference
    .generate(GenerationRequest {
        prompt: "Hello DRACARYS".to_string(),
        max_tokens: 32,
        temperature: 0.7,
    })
    .await?;

    println!("Inference test: {}", test_response.text);
    println!();

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit")
            || input.eq_ignore_ascii_case("quit")
        {
            println!("DRACARYS: Goodbye.");
            break;
        }

        let response = agent.process(input).await?;

        println!("DRACARYS: {}", response);
        println!();
    }

    Ok(())
}