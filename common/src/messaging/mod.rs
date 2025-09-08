pub mod messages {
    // Incluye el código generado por prost
    include!(concat!(env!("OUT_DIR"), "/agent_messages.rs"));
}

// Subjects NATS unificados (modo core, no JetStream)
pub const NATS_LLM_SUBJECT: &str = "agents.llm";
pub const NATS_TOOL_SUBJECT: &str = "agents.tool";
pub const NATS_COORDINATOR_SUBJECT: &str = "agents.coordinator";
pub const NATS_UI_STATUS_SUBJECT: &str = "agents.status";

