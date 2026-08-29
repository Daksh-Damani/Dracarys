mod core;
mod llm;
mod tools;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("Starting DRACARYS...");

    println!("=================================");
    println!("        DRACARYS v0.1.0");
    println!("  Local Autonomous AI Assistant");
    println!("=================================");
    println!();

    println!("Status: ONLINE");
    println!("Core:   READY");
    println!("LLM:    NOT CONNECTED");
    println!("Tools:  READY");
    println!();

    println!("DRACARYS: Hello. I'm online.");

    Ok(())
}