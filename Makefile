.PHONY: format
format:
	black tvali

.PHONY: install
install:
	pip install -e "tvali[client,api]"

.PHONY: test
test:
	docker compose \
		-f docker-compose.test.yml \
		up \
		--exit-code-from tvali_test

.PHONY: build-test
build-test:
	docker compose \
		-f docker-compose.test.yml \
		up \
		--exit-code-from tvali_test \
		--force-recreate \
		--build
