FROM rust:1.86.0 as builder
RUN useradd -ms /bin/bash thoth

RUN apt update && apt-get install protobuf-compiler libssl-dev -y
WORKDIR /home/thoth/app
RUN chown -R thoth:thoth /home/thoth/
COPY . .
RUN cargo clean
RUN cargo build --release --target-dir  thoth_binary


FROM alpine
RUN addgroup -S thoth_group && adduser -S thoth -G thoth_group

WORKDIR /home/thoth/app/
COPY --from=builder  /home/thoth/app/thoth_binary/release/thoth .
RUN chown -R thoth:thoth_group /home/thoth/app/
RUN chmod 770 thoth

USER thoth
EXPOSE 8000
CMD ["/home/thoth/app/thoth"]