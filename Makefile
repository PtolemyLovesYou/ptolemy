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

.PHONY: test
test:
	docker compose \
		-f docker-compose.test.yml \
		up \
		--exit-code-from ptolemy_test \
		--remove-orphans

.PHONY: build-test
build-test:
	docker compose \
		-f docker-compose.test.yml \
		up \
		--exit-code-from ptolemy_test \
		--force-recreate \
		--build

.PHONY: goose-clickhouse
goose-clickhouse:
	docker compose exec -e DB=clickhouse goose sh

.PHONY: goose-postgres
goose-postgres:
	docker compose exec -e DB=postgres goose sh
