FROM rust

WORKDIR /bot
COPY . .

ENTRYPOINT ["/bot/target/release/thufood-tgbot -c canteens.txt"]
