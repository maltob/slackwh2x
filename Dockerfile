FROM rust:buster AS builder
WORKDIR /workdir                       
ENV CARGO_HOME=/workdir/.cargo                       
COPY ./Cargo.toml ./Cargo.lock ./                       
COPY ./src ./src
RUN cargo build --release

FROM debian:buster
EXPOSE 8080
WORKDIR /workdir   
COPY --from=0 /workdir/target/release/slack-hook-to-x /workdir
COPY slack2x.toml /workdir
COPY data /workdir/
ENTRYPOINT ["/workdir/slack-hook-to-x"]