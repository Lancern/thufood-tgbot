extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate pretty_env_logger;
extern crate rand;
extern crate serde;
extern crate teloxide;
extern crate tokio;
extern crate toml;

mod bot;
mod canteen;

use std::path::PathBuf;

use crate::bot::Bot;
use crate::canteen::CanteenPicker;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    pretty_env_logger::init();

    let args =
        clap::App::new("Telegram bot providing canteen recommendations in Tsinghua University")
            .version("0.1.0")
            .author("Sirui Mu <msrlancern@gmail.com>")
            .arg(clap::Arg::with_name("canteens_list")
                .short("c")
                .long("canteens")
                .takes_value(true)
                .help("path to a file that contains canteens list")
                .required(true))
            .get_matches();

    let token = match std::env::var("TELEGRAM_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            log::error!("TELEGRAM_TOKEN not set");
            std::process::exit(1);
        }
    };

    let canteens_path = PathBuf::from(String::from(args.value_of("canteens_list").unwrap()));
    let canteens = match crate::canteen::load_canteens_from_file(canteens_path) {
        Ok(canteens) => canteens,
        Err(e) => {
            log::error!("Failed to load canteens list: {}", e);
            std::process::exit(1);
        }
    };
    let canteen_picker = CanteenPicker::new(canteens);

    let bot = Bot::new(token, canteen_picker);
    bot.run().await;
}
