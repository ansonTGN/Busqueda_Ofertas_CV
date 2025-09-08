.PHONY: infra-up infra-down pull-model fmt clippy run:coordinator run:llm run:tool run:ui run:all

infra-up:
\tdocker compose up -d

infra-down:
\tdocker compose down

pull-model:
\tcurl -s http://localhost:11434/api/tags >/dev/null || true
\tdocker exec -it $$(docker ps -qf name=ollama) ollama pull llama3.1:8b || true

fmt:
\tcargo fmt

clippy:
\tcargo clippy --all-targets --all-features -D warnings

run:coordinator:
\tcargo run -p agent_coordinator --bin agent_coordinator_bin

run:llm:
\tcargo run -p agent_llm --bin agent_llm

run:tool:
\tcargo run -p agent_tool --bin agent_tool_bin

run:ui:
\tcargo run -p agent_ui --bin agent_ui_bin

run:all:
\tcargo run -p launcher --bin launcher
