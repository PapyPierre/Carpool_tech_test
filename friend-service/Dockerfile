FROM rust:slim AS builder
WORKDIR /friend-service
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
LABEL authors="p.ferrari"
WORKDIR /friend-service
COPY --from=builder /friend-service/target/release/friend-service .
CMD ["./friend-service"]