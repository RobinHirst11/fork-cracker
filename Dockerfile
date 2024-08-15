FROM golang:1.19 AS builder

WORKDIR /app

COPY . .

RUN go build -o fork-cracker

FROM alpine:latest

COPY --from=builder /app/fork-cracker /app/fork-cracker

CMD ["/app/fork-cracker"]
