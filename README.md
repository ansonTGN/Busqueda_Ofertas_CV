# Sistema de Agentes Autónomos en Rust

Framework modular y escalable para orquestar **múltiples agentes** en Rust.  
Los agentes se comunican mediante **NATS** (`pub/sub`) y permiten montar pipelines que combinan **LLMs** con herramientas prácticas (PDF, web scraping, ficheros, Excel, etc.).

<p align="center">
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-1.78%2B-orange"></a>
  <a href="https://nats.io/"><img alt="NATS" src="https://img.shields.io/badge/NATS-pub/sub-blueviolet"></a>
  <a href="./LICENSE"><img alt="MIT" src="https://img.shields.io/badge/license-MIT-blue"></a>
</p>

---

## Arquitectura

- **Launcher** — arranca todos los agentes.
- **Coordinator** — enruta tareas (`agents.coordinator`).  
  Actualmente responde `OK` a cualquier request (placeholder).
- **LLM Agent** — integra modelos LLM (por defecto con [`genai`](https://crates.io/crates/genai), configurable vía `LLM_MODEL`).
- **Tool Agent** — expone herramientas:
  - **Básico (siempre disponible):**
    - `analyze_pdf` → extrae texto desde PDF con `pdf-extract`.
  - **Toolkit (opcional, activar con `--features toolkit`):**
    - `file_writer` → escribe contenido en un fichero.
    - `excel_writer` → genera informes en `.xlsx`.
    - `pdf_extractor` → extrae texto desde PDF codificado en Base64.
    - `web_search` → realiza búsquedas simples en la web (HTML scraping).
- **UI Agent** — GUI (`egui/eframe`) que muestra en tiempo real los mensajes publicados en `agents.status`.
- **Common** — tipos y utilidades compartidas (errores, constantes, proto).

**Subjects NATS por defecto:**

- `agents.coordinator`
- `agents.llm`
- `agents.tool`
- `agents.status`

---

## Requisitos

- **Rust** 1.78+ → <https://rustup.rs/>
- **Docker** (para levantar NATS y opcionalmente Ollama)
- **NATS server**

---

## Variables de entorno (`.env`)

| Variable         | Ejemplo                  | Descripción                              |
|------------------|--------------------------|------------------------------------------|
| `NATS_URL`       | `nats://127.0.0.1:4222` | URL del broker NATS                      |
| `LLM_MODEL`      | `llama3.1:8b`           | Modelo por defecto para el LLM Agent     |
| `NATS_STATUS_SUBJECT` | `agents.status`    | Subject donde la UI escucha actualizaciones |

---

## Quickstart

### 1. Levanta NATS

```bash
docker run --rm -p 4222:4222 -p 8222:8222 nats:latest
````

### 2. Arranca todo el sistema

```bash
cargo run -p launcher
```

* Se abrirá la **UI**.
* Los agentes se suscriben automáticamente a sus subjects.

### 3. Instala CLI de NATS (opcional)

Linux/macOS: [https://docs.nats.io/running-a-nats-service/nats\_tools](https://docs.nats.io/running-a-nats-service/nats_tools)

o usa el contenedor:

```bash
docker run -it --rm --network host synadia/nats-box
```

---

## Pruebas rápidas

### Coordinator

```bash
nats req agents.coordinator "ping"
# => "OK"
```

### UI

```bash
nats pub agents.status "Iniciando pipeline…"
nats pub agents.status "Procesando PDF…"
nats pub agents.status "¡Completado!"
```

La UI refleja los mensajes en vivo.

### LLM Agent

```bash
nats req agents.llm "Escríbeme un haiku corto sobre Rust en español"
```

Responde con texto generado por el modelo definido en `LLM_MODEL`.

### Tool Agent (básico)

```bash
nats req agents.tool '{"cmd":"analyze_pdf","path":"/ruta/al/archivo.pdf"}'
# o
nats req agents.tool "/ruta/al/archivo.pdf"
```

### Tool Agent (con `--features toolkit`)

Ejecuta por separado:

```bash
cargo run -p agent_tool --features toolkit
```

#### `file_writer`

```bash
nats req agents.tool '{"cmd":"file_writer","path":"./salida.md","content":"Hola desde Agent Tool"}'
```

#### `excel_writer`

```bash
nats req agents.tool '{
  "cmd":"excel_writer",
  "path":"./ofertas.xlsx",
  "jobs":[
    {"title":"Data Engineer","company":"Acme","location":"Remote","contact":"jobs@acme.com","source_url":"https://example.com/1"},
    {"title":"Rust Dev","company":"Globex","location":"Madrid","contact":"hr@globex.com","source_url":"https://example.com/2"}
  ]
}'
```

#### `web_search`

```bash
nats req agents.tool '{"cmd":"web_search","query":"rust async nats examples"}'
```

#### `pdf_extractor` (con PDF en Base64)

```bash
base64 -w0 ./cv.pdf > cv.b64
nats req agents.tool "$(jq -n --arg b64 "$(cat cv.b64)" '{cmd:"pdf_extractor", pdf_data_base64:$b64}')"
```

---

## Flujo Demo Completo

1. Publica estado inicial:

   ```bash
   nats pub agents.status "Iniciando análisis de CV…"
   ```

2. Extrae texto de un CV en PDF:

   ```bash
   nats req agents.tool '{"cmd":"analyze_pdf","path":"./cv.pdf"}'
   ```

3. Busca ofertas relacionadas:

   ```bash
   nats req agents.tool '{"cmd":"web_search","query":"Rust developer Madrid remote"}'
   ```

4. Genera Excel con resultados:

   ```bash
   nats req agents.tool '{
     "cmd":"excel_writer",
     "path":"./ofertas.xlsx",
     "jobs":[{"title":"Rust Dev","company":"Acme","location":"Remote","contact":"jobs@acme.com","source_url":"https://example.com/1"}]
   }'
   ```

5. Publica estado final:

   ```bash
   nats pub agents.status "¡Proceso completado!"
   ```

---

## Troubleshooting

* **Puertos ocupados:** cambia `NATS_URL` a `nats://127.0.0.1:4223` y lanza NATS con `-p 4223:4222`.
* **PDFs escaneados:** `pdf-extract` no hace OCR. Añade un paso con Tesseract si lo necesitas.
* **Excel corrupto:** asegúrate de no tenerlo abierto mientras se escribe.
* **Sin respuesta de agentes:** revisa que `launcher` o los binarios están corriendo y conectados a NATS.

---

## Roadmap

* [ ] Coordinator avanzado con planificador paso a paso.
* [ ] Integración OCR opcional para PDFs escaneados.
* [ ] Persistencia de estados e historiales.
* [ ] Conexión directa con portales de empleo reales.

---

## Licencia

MIT — ver [`LICENSE`](./LICENSE).


