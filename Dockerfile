FROM ubuntu:focal

WORKDIR /bot

COPY /target/release/thufood-tgbot /bot/bin/
COPY /config /bot/config

ENTRYPOINT ["/bot/bin/thufood-tgbot", "-c", "/bot/data/thufood.toml"]
