FROM rust:1.36

RUN cargo install protobuf-codegen
RUN cargo install grpcio-compiler
RUN apt-get update && apt-get -y install protobuf-compiler

WORKDIR /opt