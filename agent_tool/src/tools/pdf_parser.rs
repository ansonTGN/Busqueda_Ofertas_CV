use super::Tool;
use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use serde_json::{json, Value};

pub struct PdfParserTool;

#[derive(Deserialize)]
struct PdfParserArgs {
    pdf_data_base64: String,
}

#[async_trait]
impl Tool for PdfParserTool {
    fn name(&self) -> &'static str { "pdf_extractor" }
    fn description(&self) -> &'static str { "Extrae texto de un archivo PDF proporcionado como base64." }

    async fn execute(&self, args_json: &str) -> Result<Value> {
        let args: PdfParserArgs = serde_json::from_str(args_json)?;
        let pdf_bytes = general_purpose::STANDARD.decode(&args.pdf_data_base64)?;
        
        let text = pdf_extract::extract_text_from_mem(&pdf_bytes)?;
        
        Ok(json!({ "text": text }))
    }
}