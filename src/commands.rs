mod canteen;
mod drink;
mod help;

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
use crate::commands::help::HelpCommandHandler;
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
            create_command_handler::<HelpCommandHandler>,
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
#[command(rename = "lowercase", description = "支持的命令：")]
enum Command {
    #[command(description = "开始交互并显示帮助信息")]
    Start,

    #[command(description = "显示帮助信息")]
    Help,

    #[command(description = "随机选择一个餐厅")]
    Canteen,

    #[command(description = "线上喝奶茶")]
    Milktea,

    #[command(description = "线上喝卡布奇诺")]
    Cappuccino,

    #[command(description = "线上生产饮品")]
    ProduceDrink { drink_name: String },
}

lazy_static! {
    static ref ALL_COMMANDS: Vec<BotCommandDescriptor> = vec![
        BotCommandDescriptor {
            command: String::from("start"),
            description: String::from("开始交互并显示帮助信息"),
        },
        BotCommandDescriptor {
            command: String::from("help"),
            description: String::from("显示帮助信息"),
        },
        BotCommandDescriptor {
            command: String::from("canteen"),
            description: String::from("随机选择一个餐厅"),
        },
        BotCommandDescriptor {
            command: String::from("milktea"),
            description: String::from("线上喝奶茶"),
        },
        BotCommandDescriptor {
            command: String::from("cappuccino"),
            description: String::from("线上喝卡布奇诺"),
        },
        BotCommandDescriptor {
            command: String::from("producedrink"),
            description: String::from("线上生产饮品"),
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
