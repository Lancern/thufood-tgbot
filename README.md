# thufood-tgbot

Telegram bot that randomly picks a canteen in Tsinghua University for you!

## Build

> You need a stable Rust toolchain to build the bot. You can install a stable
> Rust toolchain in your environment with the help of
> [rustup](https://rustup.rs/).

Build the bot using `cargo`:

```shell
cargo build --release
```

After building the bot, you can build a docker image:

```shell
docker build -t thufood .
```

## Usage

### Run bot directly

```shell
export TELEGRAM_TOKEN=$TOKEN
export TELEGRAM_BOT_NAME=$NAME
cargo run --release -- -c config.yaml
```

`$TOKEN` should be set to the Telegram API token allocated for the bot. To
allocate a new token, talk to `BotFather`.

`$NAME` should be set to the Telegram bot name.

The `-c` command line option gives the path to a text file that contains the
configuration. The [`thufood.yaml`](config/thufood.yaml) file gives an example and
the default configuration.

> TODO: more documentation on the configuration schema is needed.

### Run bot in a docker container

```shell
cargo build --release
docker build .
docker run -d --net host \
    --env TELEGRAM_TOKEN=$TOKEN \
    --env TELEGRAM_BOT_NAME=$NAME \
    <image>
```

`$TOKEN` should be set to the Telegram API token allocated for the bot.

`$NAME` should be set to the Telegram bot name.

`<image>` is the identifier of the docker image produced by `docker build`. By
default, it is `thufood` as specified by the `-t` option during `docker build`.

## License

This program is open-sourced under [MIT License](./LICENSE).
