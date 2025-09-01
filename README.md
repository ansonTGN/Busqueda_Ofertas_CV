# Sistema de Agentes Autónomos en Rust

Este proyecto es una implementación de un sistema de múltiples agentes en Rust, diseñado para ser modular, escalable y robusto. Los agentes se comunican de forma asíncrona utilizando NATS y están orquestados para realizar tareas complejas que combinan el poder de los Modelos de Lenguaje Grandes (LLM) con herramientas prácticas.

##  arquitectura

El sistema se compone de varios agentes independientes, cada uno empaquetado como un crate de Rust dentro de un workspace de Cargo. Un lanzador central se encarga de iniciar todos los componentes.

-   **Launcher**: El punto de entrada del sistema. Inicia cada agente en su propio hilo con un runtime de Tokio dedicado.
-   **Agente Coordinador**: Diseñado para ser el cerebro del sistema. Escucha las peticiones de tareas y las enruta al agente apropiado (LLM o Herramientas). (Actualmente es un placeholder).
-   **Agente LLM**: Se conecta a un modelo de lenguaje (configurable para Ollama o OpenAI) para procesar y generar texto.
-   **Agente de Herramientas (Tool Agent)**: Proporciona una colección de herramientas que los otros agentes pueden utilizar. Las herramientas actuales incluyen:
    -   Extracción de texto de archivos PDF.
    -   Búsqueda y scraping web.
    -   Escritura en el sistema de archivos.
    -   Generación de archivos Excel (`.xlsx`).
-   **Agente UI**: Una interfaz gráfica de usuario sencilla construida con `egui` que muestra el estado del sistema en tiempo real, recibiendo actualizaciones a través de NATS.
-   **Common**: Un crate compartido que contiene definiciones comunes, como los mensajes Protobuf para la comunicación, los sujetos de NATS y los tipos de error.

### Flujo de Comunicación

1.  El `Launcher` inicia todos los agentes.
2.  Cada agente se suscribe a sus respectivos *subjects* en NATS.
3.  (Flujo previsto) Una tarea se origina en la UI o una API.
4.  El `Agente Coordinador` recibe la tarea y la descompone en pasos.
5.  El Coordinador envía peticiones al `Agente LLM` para razonamiento o al `Agente de Herramientas` para acciones concretas.
6.  Los agentes publican mensajes de estado en un *subject* de NATS, que son recogidos por el `Agente UI` para informar al usuario.

## Características

-   **Arquitectura Multi-agente**: Diseño desacoplado que permite que cada agente opere de forma independiente.
-   **Comunicación Asíncrona**: Uso de NATS para una comunicación eficiente y tolerante a fallos entre agentes.
-   **Integración con LLMs**: Conexión nativa con modelos locales a través de Ollama (ej. Llama 3) utilizando el crate `genai`.
-   **Herramientas Extensibles**: Un `Tool Agent` con un sistema de traits para añadir nuevas capacidades fácilmente.
-   **Interfaz de Usuario Reactiva**: GUI simple para monitorización, construida con el framework `eframe` (`egui`).
-   **Componentes reutilizables**: Un crate `common` asegura la consistencia en los mensajes y la gestión de errores.

## Primeros Pasos

### Prerrequisitos

-   **Rust**: Instala la última versión estable desde [rustup.rs](https://rustup.rs/).
-   **Docker**: Necesario para ejecutar fácilmente NATS y Ollama.

### Configuración

1.  **Clona el repositorio**:
    ```bash
    git clone <URL_DEL_REPOSITORIO>
    cd <NOMBRE_DEL_DIRECTORIO>
    ```

2.  **Configura el entorno**:
    Copia el archivo de ejemplo `test.env` a `.env`. Este archivo será leído por los agentes al iniciar.
    ```bash
    cp test.env .env
    ```
    Abre el archivo `.env` y ajusta la configuración si es necesario. Por defecto, está configurado para usar Ollama con `llama3`.

    ```dotenv
    # URL del servidor NATS
    NATS_URL="nats://localhost:4222"

    # Configuración del LLM Agent
    LLM_MODEL="llama3"
    ```

### Cómo Ejecutar

1.  **Iniciar NATS Server**:
    Ejecuta un contenedor de Docker para el servidor NATS.
    ```bash
    docker run --rm -p 4222:4222 -p 8222:8222 nats:latest
    ```

2.  **Iniciar Ollama y descargar un modelo**:
    Ejecuta el contenedor de Ollama. Si tienes una GPU NVIDIA, `--gpus=all` acelerará la inferencia.
    ```bash
    docker run -d --gpus=all -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama
    ```
    Una vez que el contenedor esté en ejecución, descarga el modelo especificado en tu `.env`:
    ```bash
    docker exec -it ollama ollama pull llama3
    ```

3.  **Ejecutar el sistema de agentes**:
    Utiliza el lanzador para compilar y ejecutar todos los agentes a la vez.
    ```bash
    cargo run --package launcher
    ```    Se abrirá la ventana del Agente UI y verás los logs de todos los agentes iniciándose en la terminal.

## Descripción de los Crates

-   `launcher`: Binario principal que coordina el lanzamiento de todos los demás agentes.
-   `common`: Crate `lib` que contiene el código compartido, como definiciones de mensajes (Protobuf), constantes de NATS y tipos de error.
-   `agent_coordinator`: Agente responsable de recibir tareas y delegarlas a otros agentes.
-   `agent_llm`: Agente que interactúa con un modelo de lenguaje grande para tareas de procesamiento de lenguaje natural.
-   `agent_tool`: Agente que expone un conjunto de herramientas prácticas, como `pdf_extractor` o `web_search`.
-   `agent_ui`: Agente que proporciona una interfaz gráfica para visualizar el estado y los resultados del sistema.