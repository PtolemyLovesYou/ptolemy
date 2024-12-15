.PHONY: format
format:
	black ptolemy \
		--exclude '\s*\.venv\s*'

.PHONY: compile-protobuf
compile-protobuf:
	cp proto/observer.proto ptolemy/proto/observer.proto
	cp proto/observer.proto observer/proto/observer.proto
	protoc -I. --include_imports -o ./vector/observer.desc ./proto/observer.proto

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
