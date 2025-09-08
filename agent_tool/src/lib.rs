use anyhow::Result;
use async_nats::Client;
use bytes::Bytes;
use common::messaging::{
    messages::{ToolRequest, ToolResponse},
    NATS_TOOL_SUBJECT,
};
use futures::StreamExt;
use prost::Message as ProstMessage;
use serde_json::Value;
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;
use tools::{
    excel_writer::ExcelWriterTool, file_system::FileSystemTool, pdf_parser::PdfParserTool,
    web_scraper::WebSearchTool, Tool,
};
use tracing::{info, warn};

pub mod tools;

pub async fn run() -> Result<()> {
    // Registro de herramientas
    let mut registry: HashMap<String, Arc<dyn Tool>> = HashMap::new();
    registry.insert("pdf_extractor".into(), Arc::new(PdfParserTool));
    registry.insert("web_search".into(), Arc::new(WebSearchTool));
    registry.insert("file_writer".into(), Arc::new(FileSystemTool));
    registry.insert("excel_writer".into(), Arc::new(ExcelWriterTool));
    let registry = Arc::new(Mutex::new(registry));

    // Conexión NATS (cliente "core")
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());
    let client = async_nats::connect(&nats_url).await?;
    info!("agent_tool conectado a NATS: {nats_url}");
    info!("Suscribiendo a '{}'", NATS_TOOL_SUBJECT);

    // Suscripción al subject canónico del proyecto
    let mut sub = client.subscribe(NATS_TOOL_SUBJECT).await?;

    while let Some(msg) = sub.next().await {
        let reply = msg.reply.clone();
        // Decodifica Protobuf de ToolRequest
        let req = match ToolRequest::decode(msg.payload.as_ref()) {
            Ok(r) => r,
            Err(e) => {
                warn!("ToolRequest malformado: {e}");
                if let Some(r) = reply {
                    let resp = ToolResponse {
                        result_json: serde_json::json!({"error": format!("bad request: {e}")}).to_string(),
                    };
                    let _ = client.publish(r, Bytes::from(resp.encode_to_vec())).await;
                }
                continue;
            }
        };

        let tool_name = req.tool_name;
        let args_json = req.arguments_json;

        // Ejecuta la herramienta
        let out: Value = {
            let map = registry.lock().await;
            match map.get(&tool_name) {
                Some(tool) => match tool.execute(&args_json).await {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("tool '{tool_name}' failed: {e}")}),
                },
                None => serde_json::json!({"error": format!("unknown tool '{tool_name}'")}),
            }
        };

        // Responde por reply inbox si existe
        if let Some(r) = reply {
            let resp = ToolResponse { result_json: out.to_string() };
            let _ = client.publish(r, Bytes::from(resp.encode_to_vec())).await;
        }
    }

    Ok(())
}


