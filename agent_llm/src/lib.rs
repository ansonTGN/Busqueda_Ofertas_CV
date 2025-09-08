use anyhow::{anyhow, Result};
use async_nats::Client as NatsClient;
use bytes::Bytes;
use common::messaging::{
    messages::{LlmRequest, LlmResponse},
    NATS_LLM_SUBJECT,
};
use futures::StreamExt;
use prost::Message as ProstMessage;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{error, info};

/// Punto de entrada del agente LLM.
/// - Se conecta a NATS.
/// - Se suscribe a `agent.llm.request`.
/// - Por cada `LlmRequest` -> llama a Ollama -> responde `LlmResponse`.
pub async fn run() -> Result<()> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".into());
    let ollama_url = env::var("OLLAMA_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
    let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3".into());

    let nc = async_nats::connect(&nats_url).await?;
    info!("agent_llm conectado a NATS: {nats_url}");
    info!("Escuchando en subject: {}", NATS_LLM_SUBJECT);
    info!("Usando Ollama en: {}  (modelo: {})", ollama_url, model);

    let mut sub = nc.subscribe(NATS_LLM_SUBJECT).await?;

    while let Some(msg) = sub.next().await {
        let reply = msg.reply.clone();

        // Decodifica el Protobuf
        let req = match LlmRequest::decode(msg.payload.as_ref()) {
            Ok(r) => r,
            Err(e) => {
                error!("LlmRequest malformado: {e}");
                continue;
            }
        };

        let sys = req.system_prompt;
        let usr = req.user_prompt;

        // Llama a Ollama
        let content = match chat_ollama(&ollama_url, &model, &sys, &usr).await {
            Ok(c) => c,
            Err(e) => {
                error!("Error Ollama: {e}");
                format!("Error: {e}")
            }
        };

        // Responde por reply-inbox (si lo hay)
        if let Some(r) = reply {
            let resp = LlmResponse { content };
            let _ = nc.publish(r, Bytes::from(resp.encode_to_vec())).await;
        }
    }

    Ok(())
}

// ------------------------- Cliente Ollama -------------------------

#[derive(Serialize)]
struct OllamaChatReq<'a> {
    model: &'a str,
    messages: Vec<OllamaMsg<'a>>,
    options: Option<OllamaOpts>,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaMsg<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize, Default)]
struct OllamaOpts {
    temperature: Option<f32>,
}

#[derive(Deserialize)]
struct OllamaChatResp {
    // Para respuestas no-stream: el campo `message` trae el texto
    message: Option<OllamaMsgOwned>,
    // Por si tu versión devuelve `response` directamente
    response: Option<String>,
}

#[derive(Deserialize)]
struct OllamaMsgOwned {
    role: String,
    content: String,
}

/// Llama al endpoint /api/chat de Ollama con un system + user.
async fn chat_ollama(base_url: &str, model: &str, system: &str, user: &str) -> Result<String> {
    let http = reqwest::Client::new();
    let url = format!("{}/api/chat", base_url);

    let body = OllamaChatReq {
        model,
        messages: vec![
            OllamaMsg { role: "system", content: system },
            OllamaMsg { role: "user", content: user },
        ],
        options: Some(OllamaOpts { temperature: Some(0.2) }),
        stream: false, // respuesta no streaming para simplificar
    };

    let resp = http.post(&url).json(&body).send().await?;
    if !resp.status().is_success() {
        return Err(anyhow!("Ollama devolvió status {}", resp.status()));
    }

    let data: OllamaChatResp = resp.json().await?;
    if let Some(m) = data.message {
        return Ok(m.content);
    }
    if let Some(s) = data.response {
        return Ok(s);
    }
    Err(anyhow!("Respuesta de Ollama sin contenido"))
}


