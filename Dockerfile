FROM lukemathwalker/cargo-chef:latest-rust-1.80.1-slim as chef
WORKDIR /app
RUN apt update && apt install lld clang -y
RUN apt install musl musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --target x86_64-unknown-linux-musl --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --target x86_64-unknown-linux-musl --release --bin zero2prod

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production

ENTRYPOINT ["./zero2prod"]
