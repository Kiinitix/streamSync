FROM rust:1.86 as builder
WORKDIR /usr/src/rapidreconcile
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/rapidreconcile/target/release/rapidreconcile /usr/local/bin/rapidreconcile
EXPOSE 8080
CMD ["/usr/local/bin/streamSync"]
