FROM rust:1-buster
RUN apt-get update && apt-get -y install libpcap-dev ca-certificates libssl-dev 
WORKDIR /usr/src/web_traffic_tracker

COPY . .

RUN cargo build --release

RUN cargo install --path .
CMD ["/usr/local/cargo/bin/web_traffic_tracker"]
