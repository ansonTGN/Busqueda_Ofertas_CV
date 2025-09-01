use anyhow::Result;
use std::path::Path;
use tokio::task;

/// Extrae el texto de un PDF usando `pdf-extract` en un hilo bloqueante.
pub async fn extract_pdf_text(path: &Path) -> Result<String> {
    let p = path.to_owned();
    let text = task::spawn_blocking(move || pdf_extract::extract_text(p))
        .await
        .map_err(|e| anyhow::anyhow!("JoinError: {e}"))??;
    Ok(text)
}
