FROM rust:buster AS builder
WORKDIR /workdir                       
ENV CARGO_HOME=/workdir/.cargo                       
COPY ./Cargo.toml ./Cargo.lock ./                       
COPY ./src ./src
RUN cargo build --release

FROM debian:buster
EXPOSE 8080
RUN apt update -y && apt install libssl1.1 ca-certificates -y && apt-get clean && update-ca-certificates
WORKDIR /app
COPY --from=0 /workdir/target/release/slack-hook-to-x /app
COPY data /app/
ENTRYPOINT ["/app/slack-hook-to-x"]