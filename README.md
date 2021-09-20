# thufood-tgbot

Telegram bot that randomly picks a canteen in Tsinghua University for you!

## Build

> You need a stable Rust toolchain to build the bot. You can install a stable Rust toolchain in your
environment with the help of [rustup](https://rustup.rs/).

Build the bot using `cargo`:

```shell
cargo build --release
```

You can also build a docker image:

```shell
docker build .
```

## Usage

### Run bot directly

```shell
export TELEGRAM_TOKEN=$TOKEN
export TELEGRAM_BOT_NAME=$NAME
cargo run --release -- -c canteens.txt
```

`$TOKEN` should be set to the Telegram API token allocated for the bot. To allocate a new token,
talk to `BotFather`.

`$NAME` should be set to the Telegram bot name.

The `-c` command line option gives the path to a text file that contains the list of canteens. The
[`canteens.txt`](./canteens.txt) file gives an example and the default configuration as to the 
canteens list. Each line within the canteen list file is a two tuple `<name>,<weight>` where 
`<name>` is the name of the canteen and `<weight>` is the "weight" of the canteen. The larger the
weight of a canteen is, the more likely the canteen will be chosen.

### Run bot in a docker container

```shell
docker build .
docker run -d --net host --env TELEGRAM_TOKEN=$TOKEN --env TELEGRAM_BOT_NAME=$NAME <image>
```

`$TOKEN` should be set to the Telegram API token allocated for the bot.

`$NAME` should be set to the Telegram bot name.

`<image>` is the identifier of the docker image produced by `docker build`.

## License

This program is open-sourced under [MIT License](./LICENSE).
