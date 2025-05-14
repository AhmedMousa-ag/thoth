FROM rust:1.86.0
RUN useradd -ms /bin/bash thoth
WORKDIR /home/thoth/app
RUN chown -R thoth:thoth /home/thoth/*
USER thoth
COPY . .
RUN cargo clean
RUN cargo build
CMD ["cargo", "run"]