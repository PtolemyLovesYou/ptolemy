.PHONY: run-unit-tests
run-unit-tests:
	python3 -m pytest \
		--cov=tvali \
		--cov-report term-missing \
		tests

.PHONY: format
format:
	python3 -m black tvali api tests

.PHONY: lint
lint:
	python3 -m pylint tvali api tests
