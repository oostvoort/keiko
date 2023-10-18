FROM node:18-alpine AS node_deps

WORKDIR /app

# Copy the package.json and yarn.lock files to the container
COPY ./dashboard/package.json ./dashboard/yarn.lock ./

# Install dependencies
RUN yarn install --frozen-lockfile

# Now copy all the sources so we can compile
FROM node:18-alpine AS node_builder
WORKDIR /app
COPY ./dashboard .
COPY --from=node_deps /app/node_modules ./node_modules

# Build the webapp
RUN yarn build --mode production

FROM rust:1 AS chef
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
WORKDIR /app/server

FROM chef AS planner

# Copy needed directories
COPY ./server/src /app/server/src
COPY ./server/Cargo.lock /app/server/Cargo.lock
COPY ./server/Cargo.toml /app/server/Cargo.toml

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

# Install DEV dependencies and others.
RUN apt-get update -y && \
    apt-get install -y net-tools build-essential python3 python3-pip valgrind

COPY --from=planner /app/server/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Copy needed directories
COPY ./server/src /app/server/src
COPY ./server/Cargo.lock /app/server/Cargo.lock
COPY ./server/Cargo.toml /app/server/Cargo.toml

# Build the binary
RUN cargo build --release

FROM oostvoort/dojo:v0.3.0

WORKDIR /opt

COPY --from=builder /app/server/target/release/server .
COPY ./server/static ./static
COPY ./server/contracts ./contracts
COPY --from=node_builder /app/dist ./static/keiko

CMD ["/opt/server"]
