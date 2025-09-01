# Proyecto: Sistema agentico para identificar ofertas adaptadas a CV

### **Estructura de Directorios a Crear**

```
/sistema-agentes-rust
|-- .gitignore
|-- Cargo.toml
|-- README.md
|-- example.env
|-- common/
|   |-- src/
|   |   |-- lib.rs
|   |   |-- error.rs
|   |   |-- task.rs
|   |   |-- messaging/
|   |   |   |-- mod.rs
|   |   |   |-- proto/
|   |   |   |   |-- agent_messages.proto
|   |-- build.rs
|   |-- Cargo.toml
|-- agent_coordinator/
|   |-- src/
|   |   |-- lib.rs
|   |   |-- main.rs
|   |-- Cargo.toml
|-- agent_llm/
|   |-- src/
|   |   |-- lib.rs
|   |   |-- main.rs
|   |-- Cargo.toml
|-- agent_tool/
|   |-- src/
|   |   |-- lib.rs
|   |   |-- main.rs
|   |   |-- tools/
|   |   |   |-- mod.rs
|   |   |   |-- excel_writer.rs
|   |   |   |-- file_system.rs
|   |   |   |-- pdf_parser.rs
|   |   |   |-- web_scraper.rs
|   |-- Cargo.toml
|-- agent_ui/
|   |-- src/
|   |   |-- lib.rs
|   |   |-- main.rs
|   |-- Cargo.toml
|-- launcher/
|   |-- src/
|   |   |-- main.rs
|   |-- Cargo.toml
```

---

### **Contenido de los Archivos**

#### **--- FILE: /README.md ---**
```markdown
# Sistema Multi-Agente Autónomo en Rust

Este proyecto es un framework para construir sistemas multi-agente autónomos en Rust. La arquitectura está diseñada para ser modular, escalable y agnóstica al proveedor de LLM. Los agentes colaboran para realizar tareas complejas, orquestados por un agente coordinador central.

## Arquitectura

El sistema se compone de varios agentes especializados que se comunican de forma asíncrona a través de un bus de mensajes NATS.

*   **Agente Coordinador (`CoordinatorAgent`)**: El cerebro del sistema. Descompone las tareas principales en planes ejecutables, delega sub-tareas a otros agentes, monitorea su progreso y ensambla los resultados finales.
*   **Agente de Interfaz de Usuario (`UIAgent`)**: Una aplicación de escritorio (`egui`) que sirve como interfaz para el usuario. Envía las tareas al coordinador y muestra el estado y los resultados.
*   **Agente de Herramientas (`ToolAgent`)**: Proporciona acceso a herramientas como búsqueda web, acceso al sistema de archivos, extracción de texto de PDFs y **creación de archivos Excel**.
*   **Agente de Inferencia LLM (`LLMAgent`)**: Abstrae la comunicación con diferentes Modelos de Lenguaje a Gran Escala (LLMs) usando la biblioteca `rust-genai`.

### Flujo de Trabajo (Ejemplo: Analizador de CV)

1.  El usuario selecciona un archivo PDF (su CV) a través del `UIAgent`.
2.  El `UIAgent` envía el contenido del PDF al `CoordinatorAgent`.
3.  El `CoordinatorAgent` utiliza un LLM para crear un plan dinámico que incluye pasos como:
    a.  Extraer el texto del PDF (`pdf_extractor`).
    b.  Analizar el texto para crear un perfil de habilidades (`LLM`).
    c.  Buscar ofertas de trabajo en la web (`web_search`).
    d.  Para cada oferta, analizarla y generar una justificación (`LLM`).
    e.  Generar un informe en Markdown con los detalles (`file_writer`).
    f.  **Generar un resumen en Excel con las ofertas (`excel_writer`)**.
4.  El `CoordinatorAgent` ejecuta cada paso, gestionando el flujo de datos entre ellos, y finalmente notifica al `UIAgent` sobre el resultado.

## Requisitos Previos

1.  **Toolchain de Rust**: [rustup.rs](https://rustup.rs/)
2.  **Servidor NATS**: `docker run -p 4222:4222 -p 8222:8222 nats:latest`
3.  **Ollama (Opcional)**: [ollama.com](https://ollama.com/) (`ollama pull llama3`)

## Configuración

Crea un archivo `.env` en la raíz del proyecto (puedes copiar `example.env`).

```env
# URL del servidor NATS
NATS_URL="nats://localhost:4222"

# Configuración del LLM Agent
LLM_PROVIDER="ollama" # "ollama" o "openai"
OLLAMA_MODEL="llama3"
# OPENAI_API_KEY="sk-..."
# OPENAI_MODEL="gpt-4-turbo"
```

## Estructura del Proyecto

El proyecto es un workspace de Cargo. Cada agente es un crate separado con su propia lógica (`lib.rs`) y punto de entrada (`main.rs`).

*   `common/`: Tipos de datos compartidos.
*   `agent_coordinator/`: Lógica del `CoordinatorAgent`.
*   `agent_ui/`: Aplicación de escritorio con `egui`.
*   `agent_tool/`: Lógica del `ToolAgent` y herramientas.
*   `agent_llm/`: Lógica del `LLMAgent` con `rust-genai`.
*   `launcher/`: Binario principal que inicia todos los agentes.

## Compilación y Ejecución

Para ejecutar el sistema completo (todos los agentes y la UI):

```bash
cargo run --bin launcher
```
```

#### **--- FILE: /Cargo.toml ---**
```toml
[workspace]
resolver = "2"
members = [
    "common",
    "agent_coordinator",
    "agent_llm",
    "agent_tool",
    "agent_ui",
    "launcher",
]

[workspace.dependencies]
# Async
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
futures = "0.3"

# Comunicación
async-nats = "0.34"
prost = "0.12"

# Serialización y Datos
serde = { version = "1", features = ["derive"] }
serde_json = "1"
bytes = "1"
base64 = "0.22"
uuid = { version = "1", features = ["v4"] }

# Errores
anyhow = "1"
thiserror = "1"

# Logging y Configuración
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
dotenv = "0.15"

# Crate común del proyecto
common = { path = "./common" }
```

#### **--- FILE: /.gitignore ---**
```
/target
.DS_Store
*.rs.bk
.env
```

#### **--- FILE: /example.env ---**
```env
# URL del servidor NATS
NATS_URL="nats://localhost:4222"

# Configuración del LLM Agent
# Proveedor puede ser "ollama" o "openai"
LLM_PROVIDER="ollama"
# Si usas ollama, especifica el modelo
OLLAMA_MODEL="llama3"
# Si usas openai, descomenta estas líneas y añade tu clave
# OPENAI_API_KEY="sk-..."
# OPENAI_MODEL="gpt-4-turbo"
```

---

### **Crate `common`**

#### **--- FILE: /common/Cargo.toml ---**
```toml
[package]
name = "common"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
prost = { workspace = true }
bytes = { workspace = true }

[build-dependencies]
prost-build = "0.12"
```

#### **--- FILE: /common/build.rs ---**
```rust
fn main() -> Result<(), std::io::Error> {
    prost_build::compile_protos(&["src/messaging/proto/agent_messages.proto"], "src/messaging/proto/")?;
    Ok(())
}
```

#### **--- FILE: /common/src/messaging/proto/agent_messages.proto ---**
```protobuf
syntax = "proto3";
package agent_messages;

// Mensaje para una petición de inferencia al LLM
message LlmRequest {
    string system_prompt = 1;
    string user_prompt = 2;
}

// Respuesta del LLM
message LlmResponse {
    string content = 1;
}

// Petición para ejecutar una herramienta
message ToolRequest {
    string tool_name = 1;
    // Argumentos en formato JSON
    string arguments_json = 2;
}

// Respuesta de una herramienta
message ToolResponse {
    // Resultado en formato JSON
    string result_json = 1;
}
```

#### **--- FILE: /common/src/lib.rs ---**
```rust
pub mod error;
pub mod messaging;
pub mod task;
```

#### **--- FILE: /common/src/error.rs ---**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Error de comunicación NATS: {0}")]
    NatsError(#[from] async_nats::Error),
    #[error("Error de serialización/deserialización: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Error de codificación/decodificación de Protobuf: {0}")]
    ProtoError(String),
    #[error("Error de I/O: {0}")]
    IoError(#[from] std::io::Error),
    #[error("La herramienta '{0}' no fue encontrada")]
    ToolNotFound(String),
    #[error("Error en la configuración: {0}")]
    Configuration(String),
    #[error("Error en la API del LLM: {0}")]
    LlmApiError(String),
    #[error("Tarea fallida: {0}")]
    TaskFailed(String),
}

impl From<prost::EncodeError> for AgentError {
    fn from(e: prost::EncodeError) -> Self {
        AgentError::ProtoError(e.to_string())
    }
}

impl From<prost::DecodeError> for AgentError {
    fn from(e: prost::DecodeError) -> Self {
        AgentError::ProtoError(e.to_string())
    }
}
```

#### **--- FILE: /common/src/task.rs ---**
```rust
use serde::{Deserialize, Serialize};

/// Representa el estado actual de una tarea compleja.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    Idle,
    InProgress(String), // Mensaje de estado
    Completed(String),  // Ruta al archivo de resultados
    Failed(String),     // Mensaje de error
}

/// Define una tarea inicial enviada por el UI al Coordinador.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitialTask {
    pub task_id: String,
    pub task_name: String,
    pub pdf_content: Vec<u8>,
}
```

#### **--- FILE: /common/src/messaging/mod.rs ---**
```rust
pub mod messages {
    // Incluye el código generado por prost
    include!(concat!(env!("OUT_DIR"), "/agent_messages.rs"));
}

pub const NATS_LLM_SUBJECT: &str = "agent.llm.request";
pub const NATS_TOOL_SUBJECT: &str = "agent.tool.request";
pub const NATS_COORDINATOR_SUBJECT: &str = "agent.coordinator.task";
pub const NATS_UI_STATUS_SUBJECT: &str = "agent.ui.status";
```

---

### **Crate `agent_llm`**

#### **--- FILE: /agent_llm/Cargo.toml ---**
```toml
[package]
name = "agent_llm"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
common = { workspace = true }
tokio = { workspace = true }
async-nats = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
dotenv = { workspace = true }
prost = { workspace = true }
bytes = { workspace = true }
futures = { workspace = true }
rust-genai = { version = "0.2", features = ["ollama-client", "openai-client"] }

[lib]
name = "agent_llm"
path = "src/lib.rs"

[[bin]]
name = "agent_llm_bin"
path = "src/main.rs"
```

#### **--- FILE: /agent_llm/src/main.rs ---**
```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    if let Err(e) = agent_llm::run().await {
        tracing::error!("El Agente LLM ha terminado con un error: {:?}", e);
    }
    
    Ok(())
}
```

#### **--- FILE: /agent_llm/src/lib.rs ---**
```rust
use anyhow::Result;
use async_nats::jetstream;
use common::error::AgentError;
use common::messaging::messages::{LlmRequest, LlmResponse};
use common::messaging::NATS_LLM_SUBJECT;
use futures::StreamExt;
use prost::Message;
use rust_genai::llm::{Llm, LlmConfig};
use rust_genai::ollama::Ollama;
use rust_genai::openai::OpenAI;
use std::env;
use std::sync::Arc;
use tracing::{error, info};

/// Contiene la lógica principal para el agente LLM.
pub async fn run() -> Result<()> {
    let provider_name = env::var("LLM_PROVIDER").unwrap_or_else(|_| "ollama".to_string());
    
    let llm: Arc<dyn Llm> = match provider_name.as_str() {
        "ollama" => {
            let config = LlmConfig::new()
                .with_model(&env::var("OLLAMA_MODEL").expect("OLLAMA_MODEL debe estar definida"));
            Arc::new(Ollama::new(config))
        }
        "openai" => {
            let config = LlmConfig::new()
                .with_model(&env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4-turbo".to_string()))
                .with_api_key(&env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY debe estar definida"));
            Arc::new(OpenAI::new(config))
        }
        _ => return Err(AgentError::Configuration("Proveedor de LLM no soportado".into()).into()),
    };
    info!("Usando el proveedor de LLM: {}", provider_name);

    let nats_url = env::var("NATS_URL").expect("NATS_URL debe estar definida");
    let nats_client = async_nats::connect(nats_url).await?;
    let jetstream = jetstream::new(nats_client);

    let mut subscription = jetstream.subscribe(NATS_LLM_SUBJECT).await?;
    info!("Escuchando peticiones de LLM en '{}'", NATS_LLM_SUBJECT);

    while let Some(msg) = subscription.next().await {
        let llm_clone = Arc::clone(&llm);
        let js_clone = jetstream.clone();
        tokio::spawn(async move {
            match LlmRequest::decode(msg.payload.clone()) {
                Ok(request) => {
                    info!("Recibida petición de LLM");
                    let result = llm_clone
                        .chat_completion(&request.system_prompt, &request.user_prompt)
                        .await;

                    let response_payload = match result {
                        Ok(content) => LlmResponse { content }.encode_to_vec(),
                        Err(e) => {
                            error!("Error al procesar la petición LLM: {:?}", e);
                            LlmResponse { content: format!("Error: {}", e) }.encode_to_vec()
                        }
                    };

                    if let Some(reply_subject) = msg.reply {
                        if let Err(e) = js_clone.publish(reply_subject, response_payload.into()).await {
                            error!("Error al enviar la respuesta: {:?}", e);
                        }
                    }
                }
                Err(e) => error!("Error al decodificar el mensaje: {:?}", e),
            }
        });
    }
    Ok(())
}
```

---

### **Crate `agent_tool`**

#### **--- FILE: /agent_tool/Cargo.toml ---**
```toml
[package]
name = "agent_tool"
version = "0.1.0"
edition = "2021"

[dependencies]
common = { workspace = true }
tokio = { workspace = true }
async-nats = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
dotenv = { workspace = true }
prost = { workspace = true }
bytes = { workspace = true }
futures = { workspace = true }
async-trait = { workspace = true }
base64 = { workspace = true }

# Dependencias específicas de herramientas
pdf-extract = "0.6"
reqwest = "0.12"
scraper = "0.19"
rust_xlsxwriter = { version = "0.7", features = ["tokio"] }

[lib]
name = "agent_tool"
path = "src/lib.rs"

[[bin]]
name = "agent_tool_bin"
path = "src/main.rs"
```

#### **--- FILE: /agent_tool/src/main.rs ---**
```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    if let Err(e) = agent_tool::run().await {
        tracing::error!("El Agente de Herramientas ha terminado con un error: {:?}", e);
    }
    
    Ok(())
}
```

#### **--- FILE: /agent_tool/src/lib.rs ---**
```rust
use anyhow::Result;
use async_nats::jetstream;
use common::messaging::messages::{ToolRequest, ToolResponse};
use common::messaging::NATS_TOOL_SUBJECT;
use futures::StreamExt;
use prost::Message;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tools::{
    excel_writer::ExcelWriterTool, file_system::FileSystemTool, pdf_parser::PdfParserTool,
    web_scraper::WebSearchTool, Tool,
};
use tracing::{error, info};

pub mod tools;

pub async fn run() -> Result<()> {
    let mut tool_registry: HashMap<String, Arc<dyn Tool>> = HashMap::new();
    tool_registry.insert("pdf_extractor".to_string(), Arc::new(PdfParserTool));
    tool_registry.insert("web_search".to_string(), Arc::new(WebSearchTool));
    tool_registry.insert("file_writer".to_string(), Arc::new(FileSystemTool));
    tool_registry.insert("excel_writer".to_string(), Arc::new(ExcelWriterTool));

    let tool_registry = Arc::new(Mutex::new(tool_registry));
    info!("Herramientas registradas: {:?}", tool_registry.lock().await.keys());

    let nats_url = env::var("NATS_URL").expect("NATS_URL debe estar definida");
    let nats_client = async_nats::connect(nats_url).await?;
    let jetstream = jetstream::new(nats_client);

    let mut subscription = jetstream.subscribe(NATS_TOOL_SUBJECT).await?;
    info!("Escuchando peticiones de herramientas en '{}'", NATS_TOOL_SUBJECT);

    while let Some(msg) = subscription.next().await {
        let registry = Arc::clone(&tool_registry);
        let js_clone = jetstream.clone();
        tokio::spawn(async move {
            match ToolRequest::decode(msg.payload.clone()) {
                Ok(request) => {
                    let tool_name = request.tool_name;
                    info!("Recibida petición para la herramienta: {}", tool_name);

                    let registry_lock = registry.lock().await;
                    let result_json = match registry_lock.get(&tool_name) {
                        Some(tool) => tool
                            .execute(&request.arguments_json)
                            .await
                            .unwrap_or_else(|e| {
                                error!("Error ejecutando la herramienta '{}': {}", tool_name, e);
                                serde_json::json!({"error": format!("Error ejecutando la herramienta: {}", e)})
                            }),
                        None => {
                            error!("Herramienta no encontrada: {}", tool_name);
                            serde_json::json!({"error": format!("Herramienta '{}' no encontrada", tool_name)})
                        }
                    };

                    let response = ToolResponse { result_json: result_json.to_string() };
                    if let Some(reply) = msg.reply {
                        if let Err(e) = js_clone.publish(reply, response.encode_to_vec().into()).await {
                            error!("Error al enviar la respuesta de la herramienta: {}", e);
                        }
                    }
                }
                Err(e) => error!("Error al decodificar petición de herramienta: {}", e),
            }
        });
    }

    Ok(())
}
```

#### **--- FILE: /agent_tool/src/tools/mod.rs ---**
```rust
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub mod excel_writer;
pub mod file_system;
pub mod pdf_parser;
pub mod web_scraper;

#[async_trait]
pub trait Tool: Send + Sync {
    /// Nombre de la herramienta
    fn name(&self) -> &'static str;
    /// Descripción de lo que hace la herramienta
    fn description(&self) -> &'static str;
    /// Ejecuta la herramienta con argumentos en formato JSON
    async fn execute(&self, args_json: &str) -> Result<Value>;
}
```

#### **--- FILE: /agent_tool/src/tools/excel_writer.rs ---**
```rust
use super::Tool;
use anyhow::Result;
use async_trait::async_trait;
use rust_xlsxwriter::{Workbook, Worksheet, XlsxError};
use serde::Deserialize;
use serde_json::{json, Value};

pub struct ExcelWriterTool;

#[derive(Deserialize, Debug)]
struct ExcelWriterArgs {
    path: String,
    jobs: Vec<JobOffer>,
}

#[derive(Deserialize, Debug)]
struct JobOffer {
    title: String,
    company: String,
    location: String,
    contact: String,
    source_url: String,
}

#[async_trait]
impl Tool for ExcelWriterTool {
    fn name(&self) -> &'static str { "excel_writer" }
    fn description(&self) -> &'static str { "Escribe una lista de ofertas de trabajo en un archivo .xlsx." }

    async fn execute(&self, args_json: &str) -> Result<Value> {
        let args: ExcelWriterArgs = serde_json::from_str(args_json)?;

        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        self.write_headers(worksheet)?;

        for (row_num, job) in args.jobs.iter().enumerate() {
            worksheet.write_string(row_num as u32 + 1, 0, &job.title)?;
            worksheet.write_string(row_num as u32 + 1, 1, &job.company)?;
            worksheet.write_string(row_num as u32 + 1, 2, &job.location)?;
            worksheet.write_string(row_num as u32 + 1, 3, &job.contact)?;
            worksheet.write_url(row_num as u32 + 1, 4, &job.source_url)?;
        }

        worksheet.autofit();
        workbook.save_async(&args.path).await?;
        
        Ok(json!({ "status": "success", "path": args.path }))
    }
}

impl ExcelWriterTool {
    fn write_headers(&self, worksheet: &mut Worksheet) -> Result<(), XlsxError> {
        worksheet.write_string(0, 0, "Puesto")?;
        worksheet.write_string(0, 1, "Empresa")?;
        worksheet.write_string(0, 2, "Ubicación")?;
        worksheet.write_string(0, 3, "Contacto")?;
        worksheet.write_string(0, 4, "Fuente")?;
        Ok(())
    }
}
```

#### **--- FILE: /agent_tool/src/tools/file_system.rs ---**
```rust
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
```

#### **--- FILE: /agent_tool/src/tools/pdf_parser.rs ---**
```rust
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
```

#### **--- FILE: /agent_tool/src/tools/web_scraper.rs ---**
```rust
use super::Tool;
use anyhow::Result;
use async_trait::async_trait;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json::{json, Value};

pub struct WebSearchTool;

#[derive(Deserialize)]
struct WebSearchArgs {
    query: String,
}

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &'static str { "web_search" }
    fn description(&self) -> &'static str { "Realiza una búsqueda web y extrae contenido de texto." }

    async fn execute(&self, args_json: &str) -> Result<Value> {
        let args: WebSearchArgs = serde_json::from_str(args_json)?;
        
        let url = format!("https://html.duckduckgo.com/html/?q={}", args.query);
        let response_html = reqwest::get(&url).await?.text().await?;
        
        let document = Html::parse_document(&response_html);
        let selector = Selector::parse("a.result__a").unwrap();
        
        let mut results = vec![];
        for element in document.select(&selector).take(5) {
            let title = element.text().collect::<String>().trim().to_string();
            if let Some(link) = element.value().attr("href") {
                 results.push(json!({"title": title, "link": link}));
            }
        }
        
        Ok(json!({ "results": results }))
    }
}
```

---

### **Crate `agent_coordinator`**

#### **--- FILE: /agent_coordinator/Cargo.toml ---**
```toml
[package]
name = "agent_coordinator"
version = "0.1.0"
edition = "2021"

[dependencies]
common = { workspace = true }
tokio = { workspace = true }
async-nats = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
dotenv = { workspace = true }
prost = { workspace = true }
bytes = { workspace = true }
futures = { workspace = true }
uuid = { workspace = true }
base64 = { workspace = true }

[lib]
name = "agent_coordinator"
path = "src/lib.rs"

[[bin]]
name = "agent_coordinator_bin"
path = "src/main.rs"
```

#### **--- FILE: /agent_coordinator/src/main.rs ---**
```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    if let Err(e) = agent_coordinator::run().await {
        tracing::error!("El Agente Coordinador ha terminado con un error: {:?}", e);
    }
    
    Ok(())
}
```

#### **--- FILE: /agent_coordinator/src/lib.rs ---**
```rust
use anyhow::{anyhow, Result};
use async_nats::{jetstream, Client};
use base64::{engine::general_purpose, Engine as _};
use common::{
    messaging::{
        messages::{LlmRequest, LlmResponse, ToolRequest, ToolResponse},
        NATS_COORDINATOR_SUBJECT, NATS_LLM_SUBJECT, NATS_TOOL_SUBJECT, NATS_UI_STATUS_SUBJECT,
    },
    task::{InitialTask, TaskStatus},
};
use futures::StreamExt;
use prost::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, env};
use tracing::{error, info, instrument};

#[derive(Serialize, Deserialize, Debug)]
struct Plan {
    steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Step {
    step_number: u32,
    description: String,
    #[serde(flatten)]
    action: Action,
    output_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action_type")]
#[serde(rename_all = "snake_case")]
enum Action {
    CallLlm {
        system_prompt: String,
        user_prompt: String,
    },
    CallTool {
        tool_name: String,
        arguments: Value,
    },
}

pub async fn run() -> Result<()> {
    let nats_url = env::var("NATS_URL").expect("NATS_URL debe estar definida");
    let nats_client = async_nats::connect(nats_url).await?;
    let jetstream = jetstream::new(nats_client);

    let mut subscription = jetstream.subscribe(NATS_COORDINATOR_SUBJECT).await?;
    info!("Escuchando tareas en '{}'", NATS_COORDINATOR_SUBJECT);

    while let Some(msg) = subscription.next().await {
        let js_clone = jetstream.clone();
        tokio::spawn(async move {
            match serde_json::from_slice::<InitialTask>(&msg.payload) {
                Ok(task) => {
                    info!("Tarea recibida: {}", task.task_id);
                    if let Err(e) = process_task(js_clone, task).await {
                        error!("Error al procesar la tarea: {:?}", e);
                    }
                }
                Err(e) => error!("Error al deserializar la tarea: {:?}", e),
            }
        });
    }
    Ok(())
}

#[instrument(skip_all, fields(task_id = task.task_id))]
async fn process_task(jetstream: jetstream::Context, task: InitialTask) -> Result<()> {
    let mut context: HashMap<String, Value> = HashMap::new();
    let task_id = task.task_id.clone();

    if let Err(e) = send_status(&jetstream, &task_id, TaskStatus::InProgress("Extrayendo texto del CV...".into())).await {
        error!("Fallo al enviar estado inicial: {}", e);
    }
    
    let pdf_base64 = general_purpose::STANDARD.encode(&task.pdf_content);
    let tool_args = serde_json::json!({ "pdf_data_base64": pdf_base64 });
    let cv_text = match call_tool(&jetstream, "pdf_extractor", tool_args).await {
        Ok(val) => val["text"].as_str().unwrap_or("").to_string(),
        Err(e) => {
            let err_msg = format!("Error al extraer PDF: {}", e);
            send_status(&jetstream, &task_id, TaskStatus::Failed(err_msg.clone())).await?;
            return Err(anyhow!(err_msg));
        }
    };
    context.insert("cv_text".to_string(), Value::String(cv_text.clone()));

    send_status(&jetstream, &task_id, TaskStatus::InProgress("Generando plan de ejecución...".into())).await?;
    let plan = match generate_plan(&jetstream, &cv_text).await {
        Ok(p) => p,
        Err(e) => {
            let err_msg = format!("Error al generar plan: {}", e);
            send_status(&jetstream, &task_id, TaskStatus::Failed(err_msg.clone())).await?;
            return Err(anyhow!(err_msg));
        }
    };

    for step in plan.steps {
        send_status(&jetstream, &task_id, TaskStatus::InProgress(step.description.clone())).await?;
        
        let interpolated_action = interpolate_action(&step.action, &context)?;
        
        let result = match interpolated_action {
            Action::CallTool { tool_name, arguments } => call_tool(&jetstream, &tool_name, arguments).await,
            Action::CallLlm { system_prompt, user_prompt } => {
                call_llm(&jetstream, &system_prompt, &user_prompt)
                    .await
                    .map(Value::String)
            }
        };

        match result {
            Ok(output) => {
                context.insert(step.output_key, output);
            }
            Err(e) => {
                let error_msg = format!("Fallo en el paso '{}': {}", step.description, e);
                send_status(&jetstream, &task_id, TaskStatus::Failed(error_msg.clone())).await?;
                return Err(anyhow!(error_msg));
            }
        }
    }
    
    let final_result_path = context
        .get("final_markdown_file_path")
        .and_then(|v| v.as_object())
        .and_then(|o| o.get("path"))
        .and_then(|p| p.as_str())
        .unwrap_or("No se generó el archivo final.")
        .to_string();

    send_status(&jetstream, &task_id, TaskStatus::Completed(final_result_path)).await?;
    info!("Tarea {} completada con éxito.", task_id);
    Ok(())
}

async fn generate_plan(jetstream: &jetstream::Context, cv_text: &str) -> Result<Plan> {
    let system_prompt = r#"
Eres un planificador experto para un sistema de agentes de software. Tu única función es generar un plan en formato JSON basado en una tarea y un contexto.
Responde SIEMPRE con un objeto JSON válido y nada más. La estructura del JSON debe ser:
{
  "steps": [
    {
      "step_number": <u32>,
      "description": "<string>",
      "action_type": "<'call_llm' o 'call_tool'>",
      // ... campos de la acción ...
      "output_key": "<string>"
    }
  ]
}

En los campos `user_prompt` y `arguments`, puedes usar placeholders `{{clave}}` para referenciar resultados de pasos anteriores guardados en el contexto bajo `output_key`.
    "#.trim().to_string();
    
    let user_prompt = format!(
        r#"
Crea un plan JSON para la tarea: "Dado el texto de un CV, encuentra online hasta 5 ofertas de trabajo adecuadas, analiza cada una, y genera un informe en Markdown y un resumen en Excel".

**Texto del CV:**
```
{}
```

**Herramientas Disponibles:**
1.  `"action_type": "call_tool", "tool_name": "web_search", "arguments": {{ "query": "búsqueda" }}`
    -   Busca en la web y devuelve una lista de títulos y URLs.
2.  `"action_type": "call_tool", "tool_name": "web_scraper", "arguments": {{ "url": "URL a scrapear" }}`
    -   Extrae el contenido de texto de una URL.
3.  `"action_type": "call_tool", "tool_name": "file_writer", "arguments": {{ "path": "ruta.md", "content": "contenido" }}`
    -   Escribe texto en un archivo.
4.  `"action_type": "call_tool", "tool_name": "excel_writer", "arguments": {{ "path": "ruta.xlsx", "jobs": [{{ "title": "...", "company": "...", "location": "...", "contact": "...", "source_url": "..." }}] }}`
    -   Escribe datos estructurados de ofertas en un archivo Excel.

**Plan Sugerido:**
1.  **Llama a un LLM** para analizar el `cv_text` y extraer habilidades, experiencia y roles ideales en formato JSON. Guarda el resultado en `cv_summary_json`.
2.  **Llama a un LLM** para generar 3 consultas de búsqueda de empleo a partir de `cv_summary_json`. Guárdalas en `search_queries_json`.
3.  **Llama a la herramienta `web_search`** usando una de las `search_queries_json`. Guarda los resultados en `job_links_json`.
4.  **Llama a un LLM** para iterar sobre los `job_links_json` y para cada link, scrapear su contenido y generar un análisis Markdown detallado comparándolo con el `cv_text`. También debe extraer datos estructurados (título, empresa, etc.). El resultado debe ser un JSON con dos claves: "markdown_reports" (un string con todos los informes concatenados) y "structured_jobs" (un array de objetos de trabajo). Guarda todo en `analysis_result_json`.
5.  **Llama a `file_writer`** para guardar el contenido de `analysis_result_json.markdown_reports` en `informe_trabajo.md`. Guarda la respuesta en `final_markdown_file_path`.
6.  **Llama a `excel_writer`** para guardar los datos de `analysis_result_json.structured_jobs` en `resumen_trabajo.xlsx`. Guarda la respuesta en `final_excel_file_path`.
"#,
        cv_text
    );

    let plan_str = call_llm(jetstream, &system_prompt, &user_prompt).await?;
    let clean_plan_str = plan_str.trim().trim_start_matches("```json").trim_end_matches("```").trim();

    serde_json::from_str(clean_plan_str).map_err(|e| anyhow!("Error al parsear el plan JSON del LLM: {}. JSON recibido: '{}'", e, clean_plan_str))
}


async fn call_llm(jetstream: &jetstream::Context, system_prompt: &str, user_prompt: &str) -> Result<String> {
    let request = LlmRequest {
        system_prompt: system_prompt.to_string(),
        user_prompt: user_prompt.to_string(),
    };
    let response_msg = jetstream.request(NATS_LLM_SUBJECT, request.encode_to_vec().into()).await?;
    let llm_response = LlmResponse::decode(response_msg.payload)?;
    Ok(llm_response.content)
}

async fn call_tool(jetstream: &jetstream::Context, tool_name: &str, arguments: Value) -> Result<Value> {
    let request = ToolRequest {
        tool_name: tool_name.to_string(),
        arguments_json: arguments.to_string(),
    };
    let response_msg = jetstream.request(NATS_TOOL_SUBJECT, request.encode_to_vec().into()).await?;
    let tool_response = ToolResponse::decode(response_msg.payload)?;
    Ok(serde_json::from_str(&tool_response.result_json)?)
}

async fn send_status(jetstream: &jetstream::Context, task_id: &str, status: TaskStatus) -> Result<()> {
    let subject = format!("{}.{}", NATS_UI_STATUS_SUBJECT, task_id);
    let payload = serde_json::to_vec(&status)?;
    jetstream.publish(subject, payload.into()).await?.await?;
    Ok(())
}

fn interpolate_string(template: &str, context: &HashMap<String, Value>) -> Result<String> {
    let mut result = template.to_string();
    for (key, value) in context {
        let placeholder = format!("{{{{{}}}}}", key);
        let value_str = match value {
            Value::String(s) => s.clone(),
            _ => value.to_string(),
        };
        result = result.replace(&placeholder, &value_str);
    }
    Ok(result)
}

fn interpolate_value(template_value: &Value, context: &HashMap<String, Value>) -> Result<Value> {
    match template_value {
        Value::String(s) => Ok(Value::String(interpolate_string(s, context)?)),
        Value::Array(arr) => {
            let new_arr = arr.iter().map(|v| interpolate_value(v, context)).collect::<Result<Vec<Value>>>()?;
            Ok(Value::Array(new_arr))
        }
        Value::Object(map) => {
            let new_map = map.iter().map(|(k, v)| Ok((k.clone(), interpolate_value(v, context)?))).collect::<Result<serde_json::Map<String, Value>>>()?;
            Ok(Value::Object(new_map))
        }
        _ => Ok(template_value.clone()),
    }
}

fn interpolate_action(action: &Action, context: &HashMap<String, Value>) -> Result<Action> {
    match action {
        Action::CallLlm { system_prompt, user_prompt } => Ok(Action::CallLlm {
            system_prompt: interpolate_string(system_prompt, context)?,
            user_prompt: interpolate_string(user_prompt, context)?,
        }),
        Action::CallTool { tool_name, arguments } => Ok(Action::CallTool {
            tool_name: tool_name.clone(),
            arguments: interpolate_value(arguments, context)?,
        }),
    }
}
```

---

### **Crate `agent_ui`**

#### **--- FILE: /agent_ui/Cargo.toml ---**
```toml
[package]
name = "agent_ui"
version = "0.1.0"
edition = "2021"

[dependencies]
common = { workspace = true }
tokio = { workspace = true }
async-nats = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
dotenv = { workspace = true }
uuid = { workspace = true }
futures = { workspace = true }
crossbeam-channel = "0.5"
eframe = { version = "0.27", features = ["persistence"] }
rfd = "0.14"

[lib]
name = "agent_ui"
path = "src/lib.rs"

[[bin]]
name = "agent_ui_bin"
path = "src/main.rs"
```

#### **--- FILE: /agent_ui/src/main.rs ---**
```rust
fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    if let Err(e) = agent_ui::run() {
        tracing::error!("El Agente UI ha terminado con un error: {}", e);
    }
}
```

#### **--- FILE: /agent_ui/src/lib.rs ---**
```rust
use anyhow::Result;
use async_nats::jetstream;
use common::task::{InitialTask, TaskStatus};
use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui;
use futures::StreamExt;
use std::{path::PathBuf, thread};
use tokio::runtime::Runtime;
use tracing::{error, info};
use uuid::Uuid;

enum UiUpdate {
    StatusChanged(TaskStatus),
}

enum NatsCommand {
    StartTask(InitialTask),
}

pub fn run() -> Result<(), eframe::Error> {
    let (ui_update_tx, ui_update_rx) = unbounded::<UiUpdate>();
    let (nats_cmd_tx, nats_cmd_rx) = unbounded::<NatsCommand>();

    thread::spawn(move || {
        let rt = Runtime::new().expect("No se pudo crear el runtime de Tokio");
        rt.block_on(nats_logic(ui_update_tx, nats_cmd_rx));
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Sistema de Agentes Autónomos",
        options,
        Box::new(|_cc| Box::new(AgentApp::new(nats_cmd_tx, ui_update_rx))),
    )
}

struct AgentApp {
    task_status: TaskStatus,
    selected_file: Option<PathBuf>,
    pdf_content: Vec<u8>,
    nats_cmd_tx: Sender<NatsCommand>,
    ui_update_rx: Receiver<UiUpdate>,
    task_id: String,
}

impl AgentApp {
    fn new(nats_cmd_tx: Sender<NatsCommand>, ui_update_rx: Receiver<UiUpdate>) -> Self {
        Self {
            task_status: TaskStatus::Idle,
            selected_file: None,
            pdf_content: Vec::new(),
            nats_cmd_tx,
            ui_update_rx,
            task_id: "".to_string(),
        }
    }
}

impl eframe::App for AgentApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(update) = self.ui_update_rx.try_recv() {
            match update {
                UiUpdate::StatusChanged(new_status) => self.task_status = new_status,
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Analizador de CV y Búsqueda de Empleo");
            ui.separator();

            if ui.button("Seleccionar CV (PDF)...").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("pdf", &["pdf"]).pick_file() {
                    match std::fs::read(&path) {
                        Ok(content) => {
                            self.pdf_content = content;
                            self.selected_file = Some(path);
                            self.task_status = TaskStatus::Idle;
                        }
                        Err(e) => self.task_status = TaskStatus::Failed(format!("Error al leer archivo: {}", e)),
                    }
                }
            }

            if let Some(path) = &self.selected_file {
                ui.label(format!("Archivo seleccionado: {}", path.display()));
            }

            let is_running = matches!(self.task_status, TaskStatus::InProgress(_));
            ui.add_enabled_ui(!self.pdf_content.is_empty() && !is_running, |ui| {
                if ui.button("Iniciar Tarea").clicked() {
                    let task_id = Uuid::new_v4().to_string();
                    self.task_id = task_id.clone();
                    let task = InitialTask {
                        task_id,
                        task_name: "AnalyzeCV".to_string(),
                        pdf_content: self.pdf_content.clone(),
                    };
                    if let Err(e) = self.nats_cmd_tx.send(NatsCommand::StartTask(task)) {
                        self.task_status = TaskStatus::Failed(format!("Error interno: {}", e));
                    } else {
                        self.task_status = TaskStatus::InProgress("Enviando tarea al coordinador...".to_string());
                    }
                }
            });
            
            ui.separator();
            ui.heading("Estado de la Tarea");
            
            match &self.task_status {
                TaskStatus::Idle => ui.label("Esperando para iniciar..."),
                TaskStatus::InProgress(status) => {
                    ui.spinner();
                    ui.label(status);
                },
                TaskStatus::Completed(result) => {
                    ui.label(format!("¡Tarea completada! Resultado en: {}", result));
                },
                TaskStatus::Failed(error) => {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                },
            };
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        });
    }
}

async fn nats_logic(ui_update_tx: Sender<UiUpdate>, nats_cmd_rx: Receiver<NatsCommand>) {
    let nats_url = std::env::var("NATS_URL").expect("NATS_URL debe estar definida");
    let client = async_nats::connect(nats_url).await.expect("No se pudo conectar a NATS");
    let jetstream = jetstream::new(client);
    
    info!("Lógica de NATS en UI iniciada.");

    loop {
        if let Ok(cmd) = nats_cmd_rx.try_recv() {
            match cmd {
                NatsCommand::StartTask(task) => {
                    let task_id = task.task_id.clone();
                    info!("Iniciando escucha para la tarea {}", task_id);
                    let status_subject = format!("{}.{}", common::messaging::NATS_UI_STATUS_SUBJECT, task_id);
                    
                    match jetstream.subscribe(status_subject).await {
                        Ok(mut sub) => {
                             let payload = serde_json::to_vec(&task).unwrap();
                            if let Err(e) = jetstream.publish(common::messaging::NATS_COORDINATOR_SUBJECT.to_string(), payload.into()).await {
                                error!("Error al publicar la tarea inicial: {}", e);
                                continue;
                            }
                            
                            let tx = ui_update_tx.clone();
                            tokio::spawn(async move {
                                while let Some(msg) = sub.next().await {
                                    if let Ok(status) = serde_json::from_slice::<TaskStatus>(&msg.payload) {
                                        let is_final_state = matches!(status, TaskStatus::Completed(_) | TaskStatus::Failed(_));
                                        if tx.send(UiUpdate::StatusChanged(status)).is_err() { break; }
                                        if is_final_state { break; }
                                    }
                                }
                                info!("Finalizada la escucha para la tarea {}", task_id);
                            });
                        }
                        Err(e) => error!("No se pudo suscribir al estado de la tarea {}: {}", task_id, e),
                    }
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}
```

---

### **Crate `launcher`**

#### **--- FILE: /launcher/Cargo.toml ---**
```toml
[package]
name = "launcher"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
dotenv = { workspace = true }

agent_coordinator = { path = "../agent_coordinator", package="agent_coordinator" }
agent_llm = { path = "../agent_llm", package="agent_llm" }
agent_tool = { path = "../agent_tool", package="agent_tool" }
agent_ui = { path = "../agent_ui", package="agent_ui" }
```

#### **--- FILE: /launcher/src/main.rs ---**
```rust
use std::thread;
use std::time::Duration;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    dotenv::dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Iniciando el sistema de agentes...");

    let agent_handles = vec![
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                info!("Iniciando Agente Coordinador...");
                if let Err(e) = agent_coordinator::run().await {
                    error!("El Agente Coordinador falló: {}", e);
                }
            });
        }),
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                info!("Iniciando Agente LLM...");
                if let Err(e) = agent_llm::run().await {
                    error!("El Agente LLM falló: {}", e);
                }
            });
        }),
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                info!("Iniciando Agente de Herramientas...");
                if let Err(e) = agent_tool::run().await {
                    error!("El Agente de Herramientas falló: {}", e);
                }
            });
        }),
    ];

    thread::sleep(Duration::from_secs(1));

    info!("Iniciando Agente de Interfaz de Usuario...");
    if let Err(e) = agent_ui::run() {
        error!("El Agente de UI falló: {}", e);
    }

    for handle in agent_handles {
        let _ = handle.join();
    }
}
```

---

### **Cómo Usar**

1.  **Crea la estructura de archivos y directorios** como se mostró al principio.
2.  **Copia y pega** el contenido de cada bloque en su archivo correspondiente.
3.  Crea un archivo `.env` en la raíz del proyecto (puedes copiar el contenido de `example.env`).
4.  Asegúrate de tener un **servidor NATS** corriendo (p. ej., con Docker).
5.  Asegúrate de tener **Ollama** corriendo si lo vas a usar.
6.  Ejecuta el lanzador desde la raíz del proyecto:
    ```bash
    cargo run --bin launcher
    ```