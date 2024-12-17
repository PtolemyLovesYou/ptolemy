.PHONY: format
format:
	black ptolemy \
		--exclude '\s*\.venv\s*'

.PHONY: run
run:
	docker compose \
		--profile dev \
		up

.PHONY: goose
goose:
	docker compose \
		exec \
		-e DB=clickhouse \
		goose \
		/bin/bash -c "source /app/configure.sh && /bin/bash"

.PHONY: diesel
diesel:
	docker compose \
		exec \
		api \
		/bin/bash -c "source /app/configure.sh && /bin/bash"
