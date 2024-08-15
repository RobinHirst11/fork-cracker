FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml ./
RUN cargo build --release

FROM alpine:latest

COPY --from=builder /app/target/release/fork-cracker /app/fork-cracker

CMD ["/app/fork-cracker"]
