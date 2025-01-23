FROM rust:1.80.0-alpine3.20 AS builder

RUN apk add --no-cache build-base openssl-dev

WORKDIR /app/

COPY Cargo.lock Cargo.toml /app/

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY . .

RUN cargo build --release

FROM rust:1.80.0-alpine3.20 AS final

COPY --from=builder /app/target/release/ctmtuci /usr/bin

CMD ["ctmtuci"]