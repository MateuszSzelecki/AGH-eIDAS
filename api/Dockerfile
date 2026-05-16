FROM rust:1.94-slim AS build

WORKDIR /app

COPY api/Cargo.toml api/Cargo.lock ./
COPY api/src ./src
COPY api/askama.toml ./
COPY zk/artifacts/vk.bin ./zk/artifacts/vk.bin

RUN cargo build --release

FROM debian:bookworm-slim AS run

WORKDIR /app

COPY --from=build /app/target/release/agh-eidas-api ./agh-eidas-api

EXPOSE 3000

CMD ["./agh-eidas-api"]
