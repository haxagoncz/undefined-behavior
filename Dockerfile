FROM rust:alpine as builder
RUN apk add --no-cache upx mold clang16 musl-dev git

ARG project=the_server_shouldnt_do_that

WORKDIR /build

COPY . .

RUN cargo build --release -p "$project" && \
    upx --best --lzma ./target/release/"$project"

FROM alpine

ARG project=the_server_shouldnt_do_that

RUN addgroup -S app && adduser -S app -G app

COPY --from=builder /build/target/release/"$project" /bin/app
RUN chmod 555 /bin/app

USER app:app

ENTRYPOINT [ "app" ]