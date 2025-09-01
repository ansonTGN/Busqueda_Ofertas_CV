use std::thread;
use std::time::Duration;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    dotenv::dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Iniciando el sistema de agentes...");

    let agent_handles = vec![
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                info!("Iniciando Agente Coordinador...");
                if let Err(e) = agent_coordinator::run().await {
                    error!("El Agente Coordinador falló: {}", e);
                }
            });
        }),
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                info!("Iniciando Agente LLM...");
                if let Err(e) = agent_llm::run().await {
                    error!("El Agente LLM falló: {}", e);
                }
            });
        }),
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                info!("Iniciando Agente de Herramientas...");
                if let Err(e) = agent_tool::run().await {
                    error!("El Agente de Herramientas falló: {}", e);
                }
            });
        }),
    ];

    thread::sleep(Duration::from_secs(1));

    info!("Iniciando Agente de Interfaz de Usuario...");
    if let Err(e) = agent_ui::run() {
        error!("El Agente de UI falló: {}", e);
    }

    for handle in agent_handles {
        let _ = handle.join();
    }
}