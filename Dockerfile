FROM rust

WORKDIR /bot
COPY . .

RUN cargo build --release

ENTRYPOINT cargo run --release -- -c canteens.txt
