use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    if let Err(e) = agent_llm::run().await {
        eprintln!("agent_llm terminó con error: {e}");
    }
    Ok(())
}
