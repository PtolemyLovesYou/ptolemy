.PHONY: format
format:
	black ptolemy prototype \
	&& black ptolemy/python \
	&& cargo fmt

.PHONY: diesel
diesel:
	docker compose exec api \
		/bin/bash -c "source /app/configure.sh && /bin/bash"

.PHONY: cli
cli:
	uv run -m ptolemy

.PHONY: generate-gql-schema
generate-gql-schema:
	OUTPUT_DIR=$(PWD)/api/graphql/schema.gql cargo run -p api --bin generate-gql-schema \
	&& OUTPUT_DIR=$(PWD)/ptolemy/graphql/schema.gql cargo run -p api --bin generate-gql-schema

.PHONY: test-client
test-client:
	cd ptolemy && cargo test --features python

.PHONY: build-client
build-client:
	maturin develop --uv -m ptolemy/Cargo.toml

.PHONY: setup-client-dev
setup-client-dev:
	uv sync --locked --dev --all-packages \
		&& make build-client \
		&& uv run -m ptolemy.setup_dev

.PHONY: docs
docs:
	uv run --directory docs -m mkdocs serve -a localhost:8080

.PHONY: run-prototype-app
run-prototype-app:
	API_URL=http://localhost:8000 uv run --directory prototype -m streamlit run app.py
