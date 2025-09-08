# Sistema de Agentes Autónomos en Rust

Framework modular y escalable para orquestar **múltiples agentes** en Rust. Los agentes se comunican por **NATS** (pub/sub) y se coordinan para ejecutar pipelines que combinan **LLMs** con **herramientas** (PDF, scraping, Excel, etc.).

<p align="center">
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-1.78%2B-orange"></a>
  <a href="https://nats.io/"><img alt="NATS" src="https://img.shields.io/badge/NATS-pub/sub-blueviolet"></a>
  <a href="./LICENSE"><img alt="MIT" src="https://img.shields.io/badge/license-MIT-blue"></a>
</p>

## Arquitectura

- **Launcher** — arranca todos los agentes.
- **Coordinator** — enruta tareas (placeholder funcional).
- **LLM Agent** — integra LLM (genai).
- **Tool Agent** — PDF, scraping, escritura de ficheros, Excel.
- **UI Agent** — GUI `eframe/egui` para estado en tiempo real.
- **Common** — tipos y utilidades (errores, messaging, proto).

**Subjects NATS:** `agents.coordinator`, `agents.llm`, `agents.tool`, `agents.status`.

## Requisitos

- Rust 1.78+
- Docker
- NATS server

## Variables de entorno

| Variable   | Ejemplo                 |
|-----------|-------------------------|
| NATS_URL  | nats://127.0.0.1:4222  |
| LLM_MODEL | llama3.1:8b            |

## Quickstart

```bash
docker run --rm -p 4222:4222 -p 8222:8222 nats:latest
cargo run --package launcher
