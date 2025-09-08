# Sistema de Agentes Autónomos en Rust

Framework multi-agente **modular, escalable y robusto** para orquestar LLMs y herramientas prácticas sobre un bus NATS.

[![Rust](https://img.shields.io/badge/Rust-2021-orange)]()
[![NATS](https://img.shields.io/badge/NATS-0.42-blueviolet)]()
[![genai](https://img.shields.io/badge/genai-0.3.5-informational)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)]()

## Arquitectura

- **Coordinator**: Descompone tareas, llama al **LLM** y al **Tool Agent**, y publica estados para la **UI**.  
- **LLM Agent**: Abstracción de modelos (vía `genai`).  
- **Tool Agent**: Registro de herramientas (PDF, scraping, filesystem, Excel).  
- **UI Agent (egui)**: Monitoriza `agent.ui.status.<task_id>` y lanza tareas.

**Subjects NATS (core request/reply + pub):**
- `agents.coordinator` (entrada de tareas)
- `agents.llm` (request/reply Protobuf `LlmRequest`→`LlmResponse`)
- `agents.tool` (request/reply Protobuf `ToolRequest`→`ToolResponse`)
- `agent.ui.status.<task_id>` (publicación de estado)

## Requisitos

- Rust estable (`rustup.rs`)
- Docker (para NATS y Ollama)

## Arranque rápido

```bash
# 1) Infra (NATS + Ollama)
docker compose up -d
# 2) (Opcional) Descargar modelo en Ollama
make pull-model
# 3) Variables de entorno
cp test.env .env
# 4) Ejecutar todo el sistema
cargo run -p launcher --bin launcher
