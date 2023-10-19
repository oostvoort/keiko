![](https://raw.githubusercontent.com/oostvoort/keiko/main/assets/logo.png)
# Keiko
Revised text:
A Vite React WebApp powers Keiko, an open-source development tool tailored for Dojo. Keiko integrates [Katana](https://book.dojoengine.org/toolchain/katana/overview.html)—an RPC controller, [Torii](https://book.dojoengine.org/toolchain/torii/overview.html)—an automatic indexer, and a runtime auto-contract deployment runner. This unified approach eliminates the need for developers to separately run Katana, Torii, and contract deployments, consolidating all processes into a single Docker container for streamlined development.

## Live Website
1. https://keiko.aw.oostvoort.work/fork
2. https://katana.keiko.aw.oostvoort.work
3. https://torii.aw.oostvoort.work



## Local Development


## Getting Started
There are three ways to set up Keiko:

### 1. Using Docker Compose (Recommended)

#### Prerequisites
1. [Docker](https://docs.docker.com/get-docker/)
2. [Docker Compose Plugin](https://docs.docker.com/compose/install/)
3. [Dojo](https://book.dojoengine.org/getting-started/quick-start.html)

#### Environment Variables
There are several environment variables that can be set for the container.
1. SERVER_PORT - the port the server is hosted (defaults to 3000)
2. KATANA_PORT - the port Katana will run from (defaults to 5050)

#### Yaml File
Copy the following Yaml file into your project's root directory.
````yaml
services:
  keiko:
    image: oostvoort/keiko:latest
    container_name: keiko
    ports:
      - "5050:5050"
      - "3000:3000"
      - "8080:8080"
      - "50051"
    restart: unless-stopped
    volumes:
      - ./contracts:/opt/contracts
    networks:
      - pixelaw

networks:
  pixelaw:
    driver: bridge

````
Mounting the contracts volume makes it so that it uses your dojo contracts instead of the 
default ones. Take note that the dojo contracts have to be compiled before starting up the
container. To compile the contracts:

````shell
# assuming that the contracts directory is in your root
cd contracts
sozo build
````

#### Setup
````shell
docker compose up -d
````

### 2. Using Docker CLI

#### Prerequisites
1. [Docker](https://docs.docker.com/get-docker/)
2. [Dojo](https://book.dojoengine.org/getting-started/quick-start.html)

#### Setup
Create the docker network
````shell
docker network create --driver bridge pixelaw
````

Run the container
````shell
docker run -d --name=keiko \
  -p 5050:5050 \
  -p 3000:3000 \
  -p 8080:8080 \
  -p 50051:50051 \
  --restart unless-stopped \
  -v $(pwd)/contracts:/opt/contracts \
  --network=pixelaw \
  oostvoort/keiko:latest
````

### 3. Running the [repository](https://github.com/oostvoort/keiko) locally

#### Prerequisites
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [NodeJS](https://nodejs.org/en/download)
3. Install [Yarn](https://classic.yarnpkg.com/lang/en/docs/install/)
4. Install [Dojo](https://book.dojoengine.org/getting-started/installation.html)

#### Setup
##### Run the server
````shell
cd server
cargo run
````
##### Install node modules
````shell
cd dashboard
yarn
````
##### Run the web app
````shell
cd dashboard
yarn dev
````