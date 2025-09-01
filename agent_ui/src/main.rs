fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    if let Err(e) = agent_ui::run() {
        tracing::error!("El Agente UI ha terminado con un error: {}", e);
    }
}