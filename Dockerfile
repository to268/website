FROM rust:alpine AS builder
WORKDIR /build

RUN apk update && \
	apk upgrade && \
	apk add pkgconfig libressl-dev musl-dev npm --no-cache

COPY rust-toolchain.toml .

RUN rustup update && \
    rustup target add wasm32-unknown-unknown && \
    cargo install --locked cargo-leptos

COPY . .

RUN cargo leptos build --release -vv

FROM alpine AS runner
WORKDIR /var/www/app

RUN addgroup -S server && \
	adduser -S www-data -G server && \
	chown -R www-data:server /var/www/app

COPY --chown=www-data:server --from=builder /build/target/release/website ./website-server
COPY --chown=www-data:server --from=builder /build/target/site ./site

USER www-data

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT "/var/www/app/site"

LABEL org.opencontainers.image.source=https://github.com/to268/website
LABEL org.opencontainers.image.description="My personal website using the Leptos framework in Rust"
LABEL org.opencontainers.image.licenses=BSD-3-Clause

EXPOSE 3000

CMD ["./website-server"]
