![Ptolemy](docs/docs/img/full-logo-lime.svg#gh-dark-mode-only)
![Ptolemy](docs/docs/img/full-logo-black.svg#gh-light-mode-only)

Universal monitoring observability for AI systems, without reinventing the wheel for each new methodology.

# Setup (Quickstart)
## Docker
To run using the quickstart docker-compose.yml, run the following command:

```sh
> docker compose up
```

You should be able to access the experimental Streamlit app at `http://localhost:8501`.

# Setup (Development)
## Run locally
To run outside of docker, you must have:
- `uv` and `cargo` installed
- a postgres instance running available at `http://localhost:5432`. If the user credentials for the Postgres instance are different than those in `.cargo/config.toml`, you can override them by exporting them as environment variables.
- a redis instance running available at `redis://localhost:6379/0`. If the credentials for the redis instance are different than those in `.cargo/config.toml`, you can override them by exporting them as environment variables.

```sh
# Start API
> cargo run -p api --bin api

# Build Python client (necessary for running the prototype frontend)
> make build-client

# If you want to run postgres and redis in docker:
> docker compose up -d postgres redis # add redis_insight if you want a redis GUI

# In a separate terminal, start experimental Streamlit frontend
> make run-prototype-app

# In a separate terminal, start the query engine
> make run-query-engine
```

# Configuration
## API
Ptolemy API has the following configurations that can be set via environment variables (you can find these in .env.example):

```env
# Postgres settings
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=ptolemy
POSTGRES_HOST=postgres
POSTGRES_PORT=5432

# Redis settings
REDIS_HOST=redis
REDIS_PORT=6379
REDIS_DB=0

# Sysadmin Credentials
PTOLEMY_USER=admin
PTOLEMY_PASS=admin

# Can be DEV, STAGE, or PROD (default is PROD)
PTOLEMY_ENV=DEV

# JWT Secret
JWT_SECRET=your-base64-encoded-secret-here

# Optional configurations
API_PORT=8000
```

For convenience, these variables are included in .cargo/config.toml as default environment variables.
