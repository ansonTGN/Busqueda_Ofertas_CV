use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub mod excel_writer;
pub mod file_system;
pub mod pdf_parser;
pub mod web_scraper;
pub mod pdf_analyzer;



#[async_trait]
pub trait Tool: Send + Sync {
    /// Nombre de la herramienta
    fn name(&self) -> &'static str;
    /// Descripción de lo que hace la herramienta
    fn description(&self) -> &'static str;
    /// Ejecuta la herramienta con argumentos en formato JSON
    async fn execute(&self, args_json: &str) -> Result<Value>;
}