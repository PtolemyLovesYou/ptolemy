.PHONY: format
format:
	black ptolemy \
		--exclude '\s*_pb2(_grpc)?.py(i)?'

.PHONY: compile-protobuf
compile-protobuf:
	python3 -m grpc_tools.protoc \
		-I. \
		--python_out=ptolemy/ \
		--pyi_out=ptolemy/ \
		--grpc_python_out=ptolemy/ \
		-o vector/observer.desc \
		proto/observer.proto
	cp proto/observer.proto observer/proto/observer.proto

.PHONY: run
run:
	docker compose up --remove-orphans

.PHONY: goose
goose:
	docker compose exec -e DB=clickhouse goose /bin/bash

.PHONY: diesel
diesel:
	docker compose exec api /bin/bash -c "source /app/configure.sh && /bin/bash"
