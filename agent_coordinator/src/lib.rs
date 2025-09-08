// agent_coordinator/src/lib.rs
use anyhow::Result;
use async_nats::Client;
use bytes::Bytes;
use futures::StreamExt;
use std::env;
use tracing::{info, warn};

/// Subject de coordinación (ajústalo si usas otros).
const SUBJECT_COORDINATOR: &str = "agents.coordinator";

/// Punto de entrada del Coordinador.
/// Se suscribe al subject de coordinación y responde "OK" a cualquier petición con reply.
/// Sustituye este loop por tu lógica real de orquestación cuando la tengas lista.
pub async fn run() -> Result<()> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());
    let client = async_nats::connect(&nats_url).await?;

    info!("Coordinator conectado a NATS en {}", nats_url);
    info!("Suscrito a '{}'", SUBJECT_COORDINATOR);

    listen_and_reply_ok(&client).await
}

async fn listen_and_reply_ok(nc: &Client) -> Result<()> {
    let mut sub = nc.subscribe(SUBJECT_COORDINATOR).await?;
    while let Some(msg) = sub.next().await {
        // Si quieres ver el payload recibido:
        // let payload = String::from_utf8_lossy(&msg.payload);
        // info!("Recibido en {}: {}", SUBJECT_COORDINATOR, payload);

        if let Some(reply) = msg.reply {
            nc.publish(reply, Bytes::from_static(b"OK")).await?;
        } else {
            // Si llega un publish sin reply, lo ignoramos (o podrías loguearlo)
            warn!("Mensaje sin 'reply' recibido en '{}'", SUBJECT_COORDINATOR);
        }
    }
    Ok(())
}

