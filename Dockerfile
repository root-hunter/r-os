FROM rust:1-alpine AS builder

WORKDIR /usr/src/app

COPY . .

RUN apk add --no-cache build-base

RUN cargo install wasm-pack
RUN wasm-pack build --target web

FROM nginx:alpine

COPY --from=builder /usr/src/app/pkg /usr/share/nginx/html/pkg
COPY --from=builder /usr/src/app/static /usr/share/nginx/html

EXPOSE 80