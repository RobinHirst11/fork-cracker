FROM rust:1.67.0-slim-buster

WORKDIR /app

COPY . .

RUN apt update && \
    apt install -y --no-install-recommends \
    curl \
    libcurl4-openssl-dev \
    openssl \
    libssl-dev \
    libncurses5-dev \
    libgtk2.0-dev \
    libglib2.0-dev \
    libreadline-dev \
    libz-dev \
    libffi-dev \
    pkg-config \
    git \
    vim \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

CMD ["/app/target/release/main"]
