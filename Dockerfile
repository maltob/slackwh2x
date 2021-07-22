FROM rust:buster AS builder
WORKDIR /workdir                       
ENV CARGO_HOME=/workdir/.cargo                       
COPY ./Cargo.toml ./Cargo.lock ./                       
COPY ./src ./src
RUN cargo build --release

FROM debian:buster
EXPOSE 8080
WORKDIR /app   
COPY --from=0 /workdir/target/release/slack-hook-to-x /app
COPY data /app/
ENTRYPOINT ["/app/slack-hook-to-x"]