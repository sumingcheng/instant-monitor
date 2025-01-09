FROM rust:1.75-slim-bullseye as builder
WORKDIR /usr/src/instant_mon

COPY Cargo.toml Cargo.lock ./

# 预构建依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src/

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY src src/
RUN cargo build --release && \
    strip target/release/instant_mon

FROM debian:bullseye-slim

# 只安装必要的运行时依赖
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/instant_mon/target/release/instant_mon /app/

ENV RUST_LOG=info

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/system/info || exit 1

EXPOSE 8080
CMD ["./instant_mon"] 