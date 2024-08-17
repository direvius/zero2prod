# FROM rust:latest AS builder

# WORKDIR /app
# RUN apt update && apt install lld clang -y
# COPY . .
# ENV SQLX_OFFLINE true
# RUN cargo build --release

# FROM rust AS runtime

# WORKDIR /app
# COPY --from=builder /app/target/release/zero2prod zero2prod
# COPY configuration configuration
# ENV APP_ENVIRONMENT production
# ENTRYPOINT ["./zero2prod"]

FROM rust:latest as builder

WORKDIR /app

RUN apt update && apt install lld clang -y
RUN apt install musl musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production

ENTRYPOINT ["./zero2prod"]
