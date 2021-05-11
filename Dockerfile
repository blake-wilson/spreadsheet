FROM rust:1-buster as builder

RUN USER=root cargo new --bin rust-docker
WORKDIR ./rust-docker
COPY ./Cargo.toml ./Cargo.toml
# COPY ./build.rs ./build.rs

RUN apt-get update \
    && apt-get install -y libssl-dev \
    && apt-get install -y build-essential \
    && apt-get install -y cmake \
    && apt-get install -y zlib1g-dev \
    && apt-get install -y protobuf-compiler

RUN cargo build --release --bin rust-docker
RUN rm src/*.rs

ADD src ./src

RUN rm ./target/release/deps/rust_docker*
RUN cargo build --release --bin spreadsheet_server

FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y libssl-dev

ARG APP=/usr/src/app

EXPOSE 9090

RUN mkdir -p ${APP}


COPY --from=builder /rust-docker/target/release/spreadsheet_server ${APP}/spreadsheet_server

WORKDIR ${APP}

CMD ["./spreadsheet_server"]
