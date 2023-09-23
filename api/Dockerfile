FROM rust:1.72-bullseye as builder
WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/anlg-api /app/anlg-api

CMD ["./anlg-api"]
