extern crate async_trait;
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate pretty_env_logger;
extern crate rand;
extern crate serde;
extern crate serde_yaml;
extern crate teloxide;
extern crate tokio;

mod commands;
mod config;
mod services;
mod utils;

use std::path::{Path, PathBuf};

use log::LevelFilter;

use crate::commands::CommandRepl;
use crate::config::Config;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let args =
        clap::App::new("Telegram bot providing canteen recommendations in Tsinghua University")
            .version("0.1.0")
            .author("Sirui Mu <msrlancern@gmail.com>")
            .arg(
                clap::Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .takes_value(true)
                    .help("path to the config file")
                    .required(true),
            )
            .arg(
                clap::Arg::with_name("verbosity")
                    .short("v")
                    .takes_value(false)
                    .multiple(true)
                    .help("verbosity of log output"),
            )
            .get_matches();

    init_logger(args.occurrences_of("verbosity"));

    let config_path = PathBuf::from(args.value_of("config").unwrap());
    let config = load_config(&config_path);

    let token = get_env_var("TELEGRAM_TOKEN");
    let bot_name = get_env_var("TELEGRAM_BOT_NAME");

    let dispatcher = match CommandRepl::from_config(&config) {
        Ok(d) => d,
        Err(e) => {
            log::error!("failed to initialize command dispatcher: {}", e);
            std::process::exit(1);
        }
    };
    dispatcher.run(token, bot_name).await;
}

fn get_env_var<T>(name: T) -> String
where
    T: AsRef<str>,
{
    let name = name.as_ref();
    match std::env::var(name) {
        Ok(value) => value,
        Err(_) => {
            log::error!("{} not set", name);
            std::process::exit(1);
        }
    }
}

fn load_config<P>(config_path: P) -> Config
where
    P: AsRef<Path>,
{
    let file_content = match std::fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => {
            log::error!("Failed to read config file: {}", e);
            std::process::exit(1);
        }
    };

    match serde_yaml::from_str(&file_content) {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to parse config file: {}", e);
            std::process::exit(1);
        }
    }
}

fn init_logger(verbosity: u64) {
    let mut builder = pretty_env_logger::formatted_builder();

    match verbosity {
        0 => builder.filter_level(LevelFilter::Warn),
        1 => builder.filter_level(LevelFilter::Info),
        2 => builder.filter_level(LevelFilter::Debug),
        _ => builder.filter_level(LevelFilter::Trace),
    };

    builder.init();
}
