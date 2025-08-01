<div align="center">

![Ptolemy](docs/docs/img/full-logo-lime.svg#gh-dark-mode-only)
![Ptolemy](docs/docs/img/full-logo-black.svg#gh-light-mode-only)

[![tests](https://github.com/PtolemyLovesYou/ptolemy/actions/workflows/test_lint.yml/badge.svg)](https://github.com/PtolemyLovesYou/ptolemy/actions/workflows/test_lint.yml)
[![docs](https://github.com/PtolemyLovesYou/ptolemy/actions/workflows/deploy_docs.yml/badge.svg)](https://github.com/PtolemyLovesYou/ptolemy/actions/workflows/deploy_docs.yml)

[![codecov](https://codecov.io/gh/PtolemyLovesYou/ptolemy/branch/main/graph/badge.svg)](https://codecov.io/gh/PtolemyLovesYou/ptolemy)
[![codspeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/PtolemyLovesYou/ptolemy)
[![Discord](https://img.shields.io/discord/1400125766246862931?label=Discord)](https://discord.gg/Grpg6BR89R)

</div>

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

```sh
# Start API
> make run-api

# Build Python client (necessary for running the prototype frontend)
> make build-client

# If you want to run postgres in docker:
> docker compose up -d postgres

# In a separate terminal, start experimental frontend
> make run-ui
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
