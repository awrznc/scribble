FROM test_app:latest AS builder
WORKDIR /usr/src/workspace/
RUN cargo build --release \
    && cargo install --path .

FROM debian:12
COPY --from=builder /usr/local/cargo/bin/docker_compose_run /usr/local/bin
