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