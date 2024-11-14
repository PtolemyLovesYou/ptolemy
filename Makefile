.PHONY: format
format:
	black tvali

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
