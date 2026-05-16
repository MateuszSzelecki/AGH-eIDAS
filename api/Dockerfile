FROM rust:1.94-slim AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY askama.toml ./

RUN cargo build --release

FROM debian:bookworm-slim AS run

WORKDIR /app

COPY --from=build /app/target/release/agh-eidas-api ./agh-eidas-api

EXPOSE 3000

CMD ["./agh-eidas-api"]
