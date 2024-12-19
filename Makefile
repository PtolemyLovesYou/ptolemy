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
