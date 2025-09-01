// agent_coordinator/src/lib.rs
use anyhow::Result;
use async_nats::jetstream;
use bytes::Bytes;
use futures::StreamExt;
use std::env;

/// Subjects (ajusta si usas otros)
const NATS_COORDINATOR_SUBJECT: &str = "agents.coordinator";
const _NATS_LLM_SUBJECT: &str = "agents.llm";
const _NATS_TOOL_SUBJECT: &str = "agents.tool";

/// Punto de entrada del coordinador.
/// Se suscribe al subject de coordinación y responde "OK" a cualquier petición con reply.
/// (Sustituye la lógica del loop por tu enrutado real si ya lo tienes definido.)
pub async fn run() -> Result<()> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());

    // Conexión core a NATS
    let client = async_nats::connect(&nats_url).await?;

    // JetStream (por si lo usas para otras operaciones)
    let _jetstream = jetstream::new(client.clone());

    // SUSCRIPCIÓN: con el cliente core, no con jetstream::Context
    let mut sub = client.subscribe(NATS_COORDINATOR_SUBJECT).await?;

    while let Some(msg) = sub.next().await {
        // Aquí podrías decodificar protobuf y enrutar a LLM/TOOLS con client.request(...)
        // De momento, confirmamos recepción respondiendo "OK" si hay reply.
        if let Some(reply) = msg.reply {
            client.publish(reply, Bytes::from_static(b"OK")).await?;
        }
    }

    Ok(())
}
