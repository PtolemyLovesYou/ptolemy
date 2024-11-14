.PHONY: format
format:
	black tvali tvali-client tvali-utils

.PHONY: install
install:
	pip install -e "tvali[client,api]"

.PHONY: test
test:
	docker compose \
		-f docker-compose.test.yml \
		up \
		--exit-code-from tvali

.PHONY: build-test
build-test:
	docker compose \
		-f docker-compose.test.yml \
		up \
		--exit-code-from tvali \
		--build
