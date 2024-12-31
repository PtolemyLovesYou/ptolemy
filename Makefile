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
	cd ptolemy && maturin develop --uv

.PHONY: docs
docs:
	uv run --directory docs -m mkdocs serve

.PHONY: run-prototype-app
run-prototype-app:
	cd ptolemy && uv run -m streamlit run app.py
