use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    if let Err(e) = agent_coordinator::run().await {
        tracing::error!("El Agente Coordinador ha terminado con un error: {:?}", e);
    }
    
    Ok(())
}