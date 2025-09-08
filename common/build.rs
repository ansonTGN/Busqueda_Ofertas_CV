use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/messaging/proto/agent_messages.proto");
    prost_build::compile_protos(
        &["src/messaging/proto/agent_messages.proto"],
        &[Path::new("src/messaging/proto")],
    )?;
    Ok(())
}

