FROM rust:1-buster
WORKDIR /usr/src/rmq-consumer

COPY . .

RUN cargo build --release

RUN cargo install --path .
CMD ["/usr/local/cargo/bin/rmq-consumer"]
