use anyhow::Result;
use async_nats::jetstream;
use bytes::Bytes;
use futures::StreamExt;
use serde::Deserialize;
use std::env;
use std::path::Path;

mod tools;
use tools::pdf_analyzer::extract_pdf_text;

const NATS_TOOL_SUBJECT: &str = "agents.tool";

#[derive(Debug, Deserialize)]
struct ToolReq {
    cmd: String,
    path: Option<String>,
}

pub async fn run() -> Result<()> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());
    let client = async_nats::connect(&nats_url).await?;
    let _js = jetstream::new(client.clone());

    let mut subscription = client.subscribe(NATS_TOOL_SUBJECT).await?;
    while let Some(msg) = subscription.next().await {
        let reply = msg.reply.clone();

        // 1) JSON {"cmd":"analyze_pdf","path":"/ruta/archivo.pdf"}
        let parsed: Result<ToolReq, _> = serde_json::from_slice(&msg.payload);

        let response = if let Ok(req) = parsed {
            match req.cmd.as_str() {
                "analyze_pdf" | "pdf_to_text" => match req.path.as_deref() {
                    Some(p) => match extract_pdf_text(Path::new(p)).await {
                        Ok(t) => t,
                        Err(e) => format!("ERROR extracting PDF: {e}"),
                    },
                    None => "ERROR: falta 'path' en la petición".to_string(),
                },
                _ => format!("ERROR: comando no reconocido '{}'", req.cmd),
            }
        } else {
            // 2) Fallback: si el payload es solo una ruta .pdf en texto plano
            match std::str::from_utf8(&msg.payload) {
                Ok(p) if p.trim_end().ends_with(".pdf") => match extract_pdf_text(Path::new(p.trim())).await {
                    Ok(t) => t,
                    Err(e) => format!("ERROR extracting PDF: {e}"),
                },
                _ => "ERROR: payload no es JSON válido ni ruta .pdf".to_string(),
            }
        };

        if let Some(r) = reply {
            let _ = client.publish(r, Bytes::from(response.into_bytes())).await;
        }
    }

    Ok(())
}

