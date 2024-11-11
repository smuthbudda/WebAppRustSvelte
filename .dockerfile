FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
CMD ["web_app_rust_svelte"]

#
# # Stage 1: Build the Rust application
# FROM rust:latest AS builder
# WORKDIR /app
# COPY . .
# RUN cargo build --release
#
# # Stage 2: Create a smaller image with only the compiled binary
# FROM debian:buster-slim
# WORKDIR /app
# # Replace 'web_app_rust_svelte' with the actual binary name from Cargo.toml if different
# COPY --from=builder /app/target/release/web_app_rust_svelte /app/web_app_rust_svelte
# CMD ["/app/web_app_rust_svelte"]
