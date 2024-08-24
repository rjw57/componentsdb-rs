ARG RUST_VERSION=1.80

FROM rust:$RUST_VERSION-slim AS base
RUN apt-get update -y && apt-get -y install libpq-dev

FROM base AS diesel-cli
RUN cargo install --no-default-features --features postgres diesel_cli
ENTRYPOINT ["diesel"]
