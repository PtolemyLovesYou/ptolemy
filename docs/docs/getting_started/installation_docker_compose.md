# Install with Docker Compose

Ptolemy is an open-source, free tool with several different deployment options. Below is a quickstart guide on how to deploy Ptolemy on your local machine with Docker.

## Deploy with Docker

To deploy Ptolemy with Docker, you must have Docker installed on your computer. See [here](https://docs.docker.com/desktop/) for instructions on how to install Docker.

First, make a directory and navigate to it:
```sh
mkdir ptolemy && cd ptolemy
```

Then, download the `docker-compose.yml` file:
```sh
wget -O docker-compose.yaml https://raw.githubusercontent.com/PtolemyLovesYou/argilla/main/docker-compose.yml
```

Set up your `.env` file by running `touch .env` and pasting the following environment variables inside:
```
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=ptolemy
POSTGRES_HOST=postgres
POSTGRES_PORT=5432
```

Run `docker compose` to start the containers:
```sh
docker compose up -d # omit the -d flag to keep the docker compose logs in your terminal
```

Once everything is up and running, run the following command to configure Postgres:
```sh
make setup
```

To verify that everything is up and running, navigate to `http://localhost:3000` in your web browser and verify that the UI loads.

### Running Prometheus
If you would like to deploy with prometheus, add the following to your `.env` file:
```
ENABLE_PROMETHEUS=true
```

Then, modify your `docker compose up` command to the following:
```
docker compose up --profile dev
```

Prometheus is disabled by default for performance reasons.
