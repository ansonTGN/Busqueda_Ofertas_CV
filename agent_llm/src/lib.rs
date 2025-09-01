// agent_llm/src/lib.rs
//! Agente LLM con genai 0.3.5 y NATS core.
//!
//! Config (variables de entorno):
//! - NATS_URL       (por defecto: nats://127.0.0.1:4222)
//! - LLM_MODEL      (por defecto: "llama3.1:8b")
//!
//! Protocolo simple (placeholder):
//! - Subject: "agents.llm"
//! - Payload de entrada: PROMPT en texto plano UTF-8
//! - Respuesta: texto generado UTF-8 vía reply inbox
//!
//! Cambia el protocolo a tu Protobuf/JSON cuando quieras: solo sustituye
//! la parte de decodificar `msg.payload` y de publicar la respuesta.

use anyhow::Result;
use bytes::Bytes;
use futures::StreamExt;
use genai::chat::{ChatMessage, ChatRequest};
use genai::{Client, ClientBuilder};
use std::env;

const NATS_LLM_SUBJECT: &str = "agents.llm";

// ---------- Cliente genai ----------

fn build_llm_client() -> Client {
    ClientBuilder::default()
        // Si necesitas endpoints custom (Azure OpenAI, Ollama remoto, etc.),
        // puedes añadir aquí un resolver con `.with_service_target_resolver_fn(...)`.
        .build()
}

async fn llm_generate_text(client: &Client, model: &str, prompt: &str) -> Result<String> {
    let req = ChatRequest::new(vec![
        ChatMessage::system("Responde en español, de forma clara y concisa."),
        ChatMessage::user(prompt),
    ]);

    // En genai 0.3.5, `exec_chat` funciona y devuelve contenido accesible con `content_text_as_str()`.
    let res = client.exec_chat(model, req, None).await?;
    Ok(res.content_text_as_str().unwrap_or_default().to_string())
}

// ---------- Worker NATS ----------

pub async fn run() -> Result<()> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());
    let model = env::var("LLM_MODEL").unwrap_or_else(|_| "llama3.1:8b".to_string());

    let nats = async_nats::connect(&nats_url).await?;
    let genai = build_llm_client();

    let mut sub = nats.subscribe(NATS_LLM_SUBJECT).await?;
    while let Some(msg) = sub.next().await {
        // Payload = prompt en UTF-8
        let prompt = match std::str::from_utf8(&msg.payload) {
            Ok(s) => s,
            Err(_) => {
                if let Some(reply) = msg.reply {
                    let _ = nats
                        .publish(reply, Bytes::from_static(b"ERROR: payload no es UTF-8"))
                        .await;
                }
                continue;
            }
        };

        let out = match llm_generate_text(&genai, &model, prompt).await {
            Ok(t) => t,
            Err(e) => format!("ERROR LLM: {e}"),
        };

        if let Some(reply) = msg.reply {
            let _ = nats.publish(reply, Bytes::from(out.into_bytes())).await;
        }
    }

    Ok(())
}
