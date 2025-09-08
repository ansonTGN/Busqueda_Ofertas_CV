pub mod messages {
    // Incluye el código generado por prost
    include!(concat!(env!("OUT_DIR"), "/agent_messages.rs"));
}

// Subjects canónicos (úsalos en todos los crates)
pub const NATS_LLM_SUBJECT: &str = "agent.llm.request";
pub const NATS_TOOL_SUBJECT: &str = "agent.tool.request";
pub const NATS_COORDINATOR_SUBJECT: &str = "agent.coordinator.task";
pub const NATS_UI_STATUS_SUBJECT: &str = "agent.ui.status";

