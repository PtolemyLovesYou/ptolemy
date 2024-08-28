.PHONY: run-unit-tests
run-unit-tests:
	python3 -m pytest \
		--cov=tvali \
		--cov-report term-missing \
		tests
