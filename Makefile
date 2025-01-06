.PHONY: format
format:
	black ptolemy prototype

.PHONY: diesel
diesel:
	docker compose exec api \
		/bin/bash -c "source /app/configure.sh && /bin/bash"

.PHONY: cli
cli:
	uv run -m ptolemy

.PHONY: generate-gql-schema
generate-gql-schema:
	cd api && cargo run --bin generate-gql-schema

.PHONY:
setup:
	docker compose exec api \
		/bin/bash -c "source /app/configure.sh && diesel migration run"

.PHONY: build-client
build-client:
	cd ptolemy && maturin develop --uv

.PHONY: docs
docs:
	uv run --directory docs -m mkdocs serve -a localhost:8080

.PHONY: run-prototype-app
run-prototype-app:
	cd ptolemy && uv run -m streamlit run app.py
