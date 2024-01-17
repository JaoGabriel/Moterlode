FROM rust:latest

WORKDIR /app/

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

CMD [ "cargo", "watch", "-x", "build", "--why" ]