use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    if let Err(e) = agent_llm::run().await {
        tracing::error!("El Agente LLM ha terminado con un error: {:?}", e);
    }
    
    Ok(())
}