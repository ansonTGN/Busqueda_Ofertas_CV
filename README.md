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