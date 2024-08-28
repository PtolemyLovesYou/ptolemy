.PHONY: run-unit-tests
run-unit-tests:
	python3 -m pytest --cov=tvali tests
