use anyhow::Result;
use async_nats::jetstream;
use bytes::Bytes;
use futures::StreamExt;
use std::collections::HashMap;
use std::env;
use std::path::Path;

mod tools;
use crate::tools::pdf_analyzer::extract_pdf_text;

const NATS_TOOL_SUBJECT: &str = "agents.tool";

pub async fn run() -> Result<()> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());
    let client = async_nats::connect(&nats_url).await?;
    let _js = jetstream::new(client.clone());

    // ---------------------------
    // Registry para `toolkit`
    // ---------------------------
    #[cfg(feature = "toolkit")]
    let registry = {
        use tools::{
            excel_writer::ExcelWriterTool, file_system::FileSystemTool, pdf_parser::PdfParserTool,
            web_scraper::WebSearchTool, Tool,
        };
        let mut map: HashMap<&'static str, Box<dyn Tool>> = HashMap::new();
        map.insert("file_writer", Box::new(FileSystemTool));
        map.insert("excel_writer", Box::new(ExcelWriterTool));
        map.insert("pdf_extractor", Box::new(PdfParserTool));
        map.insert("web_search", Box::new(WebSearchTool));
        map
    };

    let mut subscription = client.subscribe(NATS_TOOL_SUBJECT).await?;
    while let Some(msg) = subscription.next().await {
        let reply = msg.reply.clone();

        // 1) Intenta parsear como JSON {"cmd": "...", ...}
        let parsed_json: Result<serde_json::Value, _> = serde_json::from_slice(&msg.payload);
        let response = if let Ok(mut val) = parsed_json {
            // Copiamos 'cmd' a un String para no mantener un préstamo de 'val'
            let cmd_opt = val
                .get("cmd")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            match cmd_opt.as_deref() {
                // ---- Camino básico: analizar PDF por ruta ----
                Some("analyze_pdf") | Some("pdf_to_text") => {
                    let path = val.get("path").and_then(|p| p.as_str()).map(|s| s.to_string());
                    match path {
                        Some(p) => match extract_pdf_text(Path::new(&p)).await {
                            Ok(t) => t,
                            Err(e) => format!("ERROR extracting PDF: {e}"),
                        },
                        None => "ERROR: falta 'path' en la petición".to_string(),
                    }
                }

                // ---- Rutas toolkit: pasan args JSON a la herramienta correspondiente ----
                #[cfg(feature = "toolkit")]
                Some(cmd) if ["file_writer", "excel_writer", "pdf_extractor", "web_search"]
                    .contains(&cmd) =>
                {
                    use serde_json::json;

                    if let Some(obj) = val.as_object_mut() {
                        obj.remove("cmd"); // ya tenemos 'cmd' copiado
                        let args_json = json!(obj).to_string();

                        if let Some(tool) = registry.get(cmd) {
                            match tool.execute(&args_json).await {
                                Ok(value) => value.to_string(),
                                Err(e) => format!("ERROR executing tool '{cmd}': {e}"),
                            }
                        } else {
                            format!("ERROR: herramienta '{cmd}' no registrada")
                        }
                    } else {
                        format!(
                            "ERROR: el payload debe ser un objeto JSON con campos para '{cmd}'"
                        )
                    }
                }

                // ---- Comando desconocido ----
                Some(other) => format!("ERROR: comando no reconocido '{other}'"),

                // Sin campo "cmd": fallback
                None => {
                    // 2) Fallback: payload solo con ruta .pdf en texto plano
                    match std::str::from_utf8(&msg.payload) {
                        Ok(p) if p.trim_end().ends_with(".pdf") => {
                            match extract_pdf_text(Path::new(p.trim())).await {
                                Ok(t) => t,
                                Err(e) => format!("ERROR extracting PDF: {e}"),
                            }
                        }
                        _ => "ERROR: payload sin 'cmd' no es JSON válido ni ruta .pdf".to_string(),
                    }
                }
            }
        } else {
            // 3) Fallback total: payload no es JSON -> prueba como ruta .pdf
            match std::str::from_utf8(&msg.payload) {
                Ok(p) if p.trim_end().ends_with(".pdf") => {
                    match extract_pdf_text(Path::new(p.trim())).await {
                        Ok(t) => t,
                        Err(e) => format!("ERROR extracting PDF: {e}"),
                    }
                }
                _ => "ERROR: payload no es JSON válido ni ruta .pdf".to_string(),
            }
        };

        if let Some(r) = reply {
            let _ = client.publish(r, Bytes::from(response.into_bytes())).await;
        }
    }
    Ok(())
}


