FROM rust:1.94-slim AS build

WORKDIR /app

COPY api/ ./api/
COPY zk/artifacts/vk.bin ./zk/artifacts/vk.bin

WORKDIR /app/api

RUN cargo build --release

FROM debian:bookworm-slim AS run

WORKDIR /app

COPY --from=build /app/api/target/release/agh-eidas-api ./agh-eidas-api

EXPOSE 3000

CMD ["./agh-eidas-api"]
