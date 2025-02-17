# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.84.0
ARG APP_NAME=yuimarl

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /src

RUN apk update && apk add --no-cache clang lld musl-dev git make

COPY . .
RUN cargo build --release
COPY ./static /src/static

FROM alpine:3.18 AS final
WORKDIR /src

COPY --from=build /src/target/release/yuimarl /src/yuimarl
COPY --from=build /src/static /src/static

EXPOSE 8080

CMD ["/src/yuimarl"]
