FROM rust:1.80 AS builder
WORKDIR /usr/src/optiq-ai
COPY backend/ .
RUN cargo build --release --bin plansense

FROM debian:bookworm-slim
COPY --from=builder /usr/src/optiq-ai/target/release/plansense /usr/local/bin/plansense
EXPOSE 8082
CMD ["plansense"]