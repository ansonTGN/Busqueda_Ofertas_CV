// agent_tool/src/tools/mod.rs

// Siempre disponible en modo básico: extractor síncrono en hilo
pub mod pdf_analyzer;

// --- Herramientas opcionales (solo se compilan si activas --features toolkit) ---

#[cfg(feature = "toolkit")]
pub mod excel_writer;

#[cfg(feature = "toolkit")]
pub mod file_system;

#[cfg(feature = "toolkit")]
pub mod pdf_parser;

#[cfg(feature = "toolkit")]
pub mod web_scraper;

#[cfg(feature = "toolkit")]
use anyhow::Result;

#[cfg(feature = "toolkit")]
use async_trait::async_trait;

#[cfg(feature = "toolkit")]
use serde_json::Value;

/// Trait de herramientas solo cuando está activo `toolkit`
/// (puede no usarse directamente si no consultas `name/description`).
#[cfg(feature = "toolkit")]
#[allow(dead_code)]
#[async_trait]
pub trait Tool: Send + Sync {
    /// Nombre de la herramienta
    fn name(&self) -> &'static str;
    /// Descripción de lo que hace la herramienta
    fn description(&self) -> &'static str;
    /// Ejecuta la herramienta con argumentos en formato JSON
    async fn execute(&self, args_json: &str) -> Result<Value>;
}

