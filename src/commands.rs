mod canteen;
mod drink;
mod help;
mod meow;
mod twd2;

use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use teloxide::adaptors::AutoSend;
use teloxide::dispatching::UpdateWithCx;
use teloxide::requests::{Requester, RequesterExt};
use teloxide::types::{BotCommand as BotCommandDescriptor, Message};
use teloxide::utils::command::BotCommand;
use teloxide::Bot;

use crate::commands::canteen::CanteenCommandHandler;
use crate::commands::drink::DrinkCommandHandler;
use crate::commands::help::HelpCommandHandler;
use crate::commands::meow::MeowCommandHandler;
use crate::commands::twd2::Twd2CommandHandler;
use crate::config::Config;

/// A command REPL bot that serves the thufood bot commands in a REPL.
#[derive(Clone, Default)]
pub struct CommandRepl {
    handlers: Vec<Arc<dyn CommandHandler>>,
}

impl CommandRepl {
    /// Create a new `CommandRepl` from the given application configuration.
    pub fn from_config(config: &Config) -> Result<Arc<Self>, Box<dyn Error>> {
        fn create_command_handler<H>(
            config: &Config,
        ) -> Result<Arc<dyn CommandHandler>, Box<dyn Error>>
        where
            H: 'static + CommandHandler,
        {
            let handler = H::new(config)?;
            Ok(Arc::new(handler))
        }

        let mut dispatcher = Self::default();

        let factories = [
            create_command_handler::<CanteenCommandHandler>,
            create_command_handler::<DrinkCommandHandler>,
            create_command_handler::<HelpCommandHandler>,
            create_command_handler::<MeowCommandHandler>,
            create_command_handler::<Twd2CommandHandler>,
        ];
        for f in factories {
            let handler = f(config)?;
            dispatcher.handlers.push(handler);
        }

        Ok(Arc::new(dispatcher))
    }

    /// Run the REPL.
    pub async fn run(self: Arc<Self>, token: String, name: String) {
        let bot = Bot::new(token).auto_send();

        // Register all the commands provided by the bot.
        match bot.set_my_commands(ALL_COMMANDS.clone()).await {
            Ok(_) => (),
            Err(e) => {
                log::warn!("Cannot set commands: {}", e);
            }
        }

        let self_share = self.clone();
        teloxide::commands_repl(bot, name, move |ctx, cmd| {
            Self::handle_message(self_share.clone(), ctx, cmd)
        })
        .await;
    }

    async fn handle_message(
        self: Arc<Self>,
        ctx: UpdateWithCx<AutoSend<Bot>, Message>,
        cmd: Command,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        for h in &self.handlers {
            if h.clone().accept(&cmd) {
                h.clone().handle(ctx, cmd).await?;
                break;
            }
        }
        Ok(())
    }
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "??????????????????")]
enum Command {
    #[command(description = "?????????????????????????????????")]
    Start,

    #[command(description = "??????????????????")]
    Help,

    #[command(description = "????????????????????????")]
    Canteen,

    #[command(description = "???????????????")]
    Milktea,

    #[command(description = "?????????????????????")]
    Cappuccino,

    #[command(description = "??????????????????")]
    ProduceDrink { drink_name: String },

    #[command(description = "????????????")]
    FeedMeow,

    #[command(description = "?????? WD40 ????????????")]
    FeedMeowWd40,

    #[command(description = "????????????????????????????????????")]
    FeedTwd2,
}

lazy_static! {
    static ref ALL_COMMANDS: Vec<BotCommandDescriptor> = vec![
        BotCommandDescriptor {
            command: String::from("start"),
            description: String::from("?????????????????????????????????"),
        },
        BotCommandDescriptor {
            command: String::from("help"),
            description: String::from("??????????????????"),
        },
        BotCommandDescriptor {
            command: String::from("canteen"),
            description: String::from("????????????????????????"),
        },
        BotCommandDescriptor {
            command: String::from("milktea"),
            description: String::from("???????????????"),
        },
        BotCommandDescriptor {
            command: String::from("cappuccino"),
            description: String::from("?????????????????????"),
        },
        BotCommandDescriptor {
            command: String::from("producedrink"),
            description: String::from("??????????????????"),
        },
        BotCommandDescriptor {
            command: String::from("feedmeow"),
            description: String::from("????????????"),
        },
        BotCommandDescriptor {
            command: String::from("feedmeowwd40"),
            description: String::from("?????? WD40 ????????????"),
        },
        BotCommandDescriptor {
            command: String::from("feedtwd2"),
            description: String::from("????????????????????????????????????"),
        }
    ];
}

#[async_trait]
trait CommandHandler: Send + Sync {
    fn new(config: &Config) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    fn accept(self: Arc<Self>, cmd: &Command) -> bool;

    async fn handle(
        self: Arc<Self>,
        ctx: UpdateWithCx<AutoSend<Bot>, Message>,
        cmd: Command,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}
