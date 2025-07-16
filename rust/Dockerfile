# -----------------------
# 1. Build stage
# -----------------------
FROM rust:latest as builder

WORKDIR /app

COPY . .
RUN cargo build --release

# 2. Runtime stage
FROM rust:latest

WORKDIR /app

COPY --from=builder /app/target/release/server ./server

COPY . .

EXPOSE 8080

# Run server or test
CMD if [ "$RUST_SERVER_TEST" = "0" ]; then \
      echo "Starting server..." && ./server ; \
    else \
      echo "Build is skipped because it's tests run" && \
      echo "Running tests..." && cargo test --release -- --nocapture ; \
    fi