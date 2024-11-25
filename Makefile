.PHONY: format
format:
	black tvali

.PHONY: compile-protobuf
compile-protobuf:
	python3 -m grpc_tools.protoc \
		-Iproto/ \
		--python_out=tvali/tvali/proto \
		--pyi_out=tvali/tvali/proto \
		--grpc_python_out=tvali/tvali/proto \
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
		--exit-code-from tvali_test \
		--remove-orphans

.PHONY: build-test
build-test:
	docker compose \
		-f docker-compose.test.yml \
		up \
		--exit-code-from tvali_test \
		--force-recreate \
		--build
