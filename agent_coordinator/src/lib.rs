// agent_coordinator/src/lib.rs
use anyhow::Result;
use async_nats::jetstream;
use bytes::Bytes;
use futures::StreamExt;
use std::env;

/// Subjects (core)
const NATS_COORDINATOR_SUBJECT: &str = "agents.coordinator";
const _NATS_LLM_SUBJECT: &str = "agents.llm";
const _NATS_TOOL_SUBJECT: &str = "agents.tool";

/// Coordinador básico: responde "OK" a cualquier request con reply.
/// Sustituye este loop por tu enrutado real cuando quieras.
pub async fn run() -> Result<()> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());

    let client = async_nats::connect(&nats_url).await?;
    let _jetstream = jetstream::new(client.clone());

    let mut sub = client.subscribe(NATS_COORDINATOR_SUBJECT).await?;
    while let Some(msg) = sub.next().await {
        if let Some(reply) = msg.reply {
            client.publish(reply, Bytes::from_static(b"OK")).await?;
        }
    }
    Ok(())
}

