# Static builder for Rust
FROM ekidd/rust-musl-builder AS builder
COPY . /home/rust/src
RUN cargo build --release

# New image from scratch (minimal sized)
FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/order-server /order-server
EXPOSE 3002
ENTRYPOINT ["/order-server"]