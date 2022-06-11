FROM rust:latest

ADD core ./core
ADD get_str ./get_str
ADD Cargo.toml Cargo.toml
RUN cargo build

