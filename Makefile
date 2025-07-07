.PHONY: format
format:
	black --pyi ptolemy-py/python integration-tests \
	&& cargo fmt

.PHONY: diesel
diesel:
	docker compose exec api \
		/bin/bash -c "source /app/configure.sh && /bin/bash"

.PHONY: cli
cli:
	uv run -m ptolemy_client

.PHONY: generate-gql-schema
generate-gql-schema:
	OUTPUT_DIR=$(PWD)/api/graphql/schema.gql cargo run -p api --bin generate-gql-schema \
	&& OUTPUT_DIR=$(PWD)/ptolemy/graphql/schema.gql cargo run -p api --bin generate-gql-schema

.PHONY: test-client
test-client:
	cd ptolemy && cargo test --features python

.PHONY: build-client
build-client:
	maturin develop --uv -m ptolemy-py/Cargo.toml

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

.PHONY: run-query-engine
run-query-engine:
	uv run --directory query-engine main.py

.PHONY: run-integration_tests
run-integration-tests:
	uv run --directory integration-tests -m pytest integration_tests
