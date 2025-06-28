FROM rust:1.80 AS builder
WORKDIR /usr/src/optiq-ai
COPY backend/ .
RUN cargo build --release --bin wokcore

FROM debian:bookworm-slim
COPY --from=builder /usr/src/optiq-ai/target/release/wokcore /usr/local/bin/wokcore
EXPOSE 8081
CMD ["wokcore"]