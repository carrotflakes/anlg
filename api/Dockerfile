FROM rust:1.72 as builder
WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/anlg-api /app/anlg-api

CMD ["./anlg-api"]
