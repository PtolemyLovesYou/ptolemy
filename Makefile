.PHONY: format
format:
	black ptolemy \
		--exclude '\s*\.venv\s*'

.PHONY: migrate
migrate:
	docker compose \
		exec \
		-e DB=clickhouse \
		api \
		/bin/bash -c "source /app/configure.sh && /bin/bash"
