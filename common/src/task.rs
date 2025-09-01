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