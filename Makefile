.PHONY: format
format:
	black ptolemy

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
