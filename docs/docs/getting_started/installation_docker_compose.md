# Install with Docker Compose

Ptolemy is an open-source, free tool with several different deployment options. Below is a quickstart guide on how to deploy Ptolemy on your local machine with Docker.

## Deploy with Docker

To deploy Ptolemy with Docker, you must have Docker installed on your computer. See [here](https://docs.docker.com/desktop/) for instructions on how to install Docker.

First, make a directory and navigate to it:
```sh
mkdir ptolemy-quickstart && cd ptolemy-quickstart
```

Then, download the `docker-compose.yml` file:
```sh
wget -O docker-compose.yml https://raw.githubusercontent.com/PtolemyLovesYou/ptolemy/main/docker-compose.quickstart.yml
```

Run `docker compose` to start the containers:
```sh
docker compose up -d # omit the -d flag to keep the docker compose logs in your terminal
```

To verify that everything is up and running, navigate to `http://localhost:8501` in your web browser and verify that the UI loads.
