FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN cargo run

FROM alpine:latest

COPY --from=builder /app/target/release/fork-cracker /app/fork-cracker

CMD ["/app/fork-cracker"]
