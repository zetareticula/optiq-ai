FROM rust:1.80 AS builder
WORKDIR /usr/src/optiq-ai
COPY api/ .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/optiq-ai/target/release/optiq-api /usr/local/bin/optiq-api
EXPOSE 8080
CMD ["optiq-api"]