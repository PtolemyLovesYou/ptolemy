.PHONY: format
format:
	uv run -m black --pyi ptolemy-py/python ptolemy-py/tests ptolemy-py/examples \
	&& cargo fmt

.PHONY: diesel
diesel:
	docker compose exec api \
		/bin/bash -c "source /app/configure.sh && /bin/bash"

.PHONY: test-client
test-client:
	uv run -m pytest ptolemy-py/tests --verbose

.PHONY: benchmark-client
benchmark-client:
	uv run -m pytest ptolemy-py/tests --codspeed

.PHONY: build-client
build-client:
	uv run -m maturin develop --uv -m ptolemy-py/Cargo.toml

.PHONY: setup-client-dev
setup-client-dev:
	uv sync --locked --dev --all-packages \
		&& make build-client \
		&& uv run -m ptolemy.setup_dev

.PHONY: docs
docs:
	uv run --directory docs -m mkdocs serve -a localhost:8080

.PHONY: run-api
run-api:
	cargo run -p api --bin api

.PHONY: run-ui
run-ui:
	VITE_PTOLEMY_API=http://localhost:8000 VITE_PTOLEMY_DOCS=http://localhost:8080 cd ptolemy-ui && npm install --force && npm run dev
