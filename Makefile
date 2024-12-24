.PHONY: format
format:
	black ptolemy \
		--exclude '\s*\.venv\s*'

.PHONY: diesel
diesel:
	docker compose exec api \
		/bin/bash -c "source /app/configure.sh && /bin/bash"


.PHONY:
setup:
	docker compose exec api \
		/bin/bash -c "source /app/configure.sh && diesel migration run"

.PHONY: build-client
build-client:
	cd ptolemy && maturin develop

.PHONY: install-client
install-client:
	make build-client \
		&& pip install -e ./ptolemy

.PHONY: docs
docs:
	cd docs && uv run mkdocs serve

.PHONY: run-prototype-app
run-prototype-app:
	VIRTUAL_ENV=$(pwd)/app-prototype/.venv cd app-prototype && uv run -m streamlit run main.py