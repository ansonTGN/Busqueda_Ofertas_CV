// common/build.rs
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Vuelve a ejecutar el build script si cambia el .proto
    println!("cargo:rerun-if-changed=src/messaging/proto/agent_messages.proto");

    // prost-build 0.12: ambos parámetros son slices (&[...])
    prost_build::compile_protos(
        &["src/messaging/proto/agent_messages.proto"],
        &[Path::new("src/messaging/proto")],
    )?;

    Ok(())
}

