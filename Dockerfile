FROM rust:1.87.0-alpine as builder

RUN apk add --no-cache musl-dev protobuf-dev openssl-dev openssl-libs-static

RUN addgroup -S thoth_group && adduser -S thoth -G thoth_group

WORKDIR /home/thoth/app
RUN chown -R thoth:thoth_group /home/thoth/
COPY . .
RUN chown -R thoth:thoth_group .

RUN cargo clean
RUN cargo build --release --target-dir thoth_binary

FROM alpine
RUN addgroup -S thoth_group && adduser -S thoth -G thoth_group

WORKDIR /home/thoth/app/
COPY --from=builder /home/thoth/app/thoth_binary/release/thoth .
RUN chown -R thoth:thoth_group /home/thoth/app/
RUN chmod +x thoth

USER thoth
EXPOSE 8000
CMD ["/home/thoth/app/thoth"]