use super::Tool;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::fs;

pub struct FileSystemTool;

#[derive(Deserialize)]
struct FileWriterArgs {
    path: String,
    content: String,
}

#[async_trait]
impl Tool for FileSystemTool {
    fn name(&self) -> &'static str { "file_writer" }
    fn description(&self) -> &'static str { "Escribe contenido en un archivo en el sistema de ficheros." }

    async fn execute(&self, args_json: &str) -> Result<Value> {
        let args: FileWriterArgs = serde_json::from_str(args_json)?;
        fs::write(&args.path, &args.content).await?;
        Ok(json!({ "status": "success", "path": args.path }))
    }
}