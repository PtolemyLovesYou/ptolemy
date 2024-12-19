.PHONY: format
format:
	black ptolemy \
		--exclude '\s*\.venv\s*'

.PHONY: diesel
diesel:
	docker compose \
		exec \
		-e DB=clickhouse \
		api \
		/bin/bash -c "source /app/configure.sh && /bin/bash"

.PHONY: build-client
build-client:
	cd ptolemy && maturin develop

.PHONY: install-client
install-client:
	make build-client \
		&& pip install -e ./ptolemy

.PHONY: docs
docs:
	cd docs && uv run mkdocs serve
