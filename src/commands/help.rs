use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use teloxide::adaptors::AutoSend;
use teloxide::dispatching::UpdateWithCx;
use teloxide::types::Message;
use teloxide::utils::command::BotCommand;
use teloxide::Bot;

use crate::commands::{Command, CommandHandler};
use crate::config::Config;

/// Handler of the `/start` and the `/help` commands.
#[derive(Clone, Debug, Default)]
pub struct HelpCommandHandler;

#[async_trait]
impl CommandHandler for HelpCommandHandler {
    fn new(_config: &Config) -> Result<Self, Box<dyn Error>> {
        Ok(Self)
    }

    fn accept(self: Arc<Self>, cmd: &Command) -> bool {
        match cmd {
            Command::Help | Command::Start => true,
            _ => false,
        }
    }

    async fn handle(
        self: Arc<Self>,
        ctx: UpdateWithCx<AutoSend<Bot>, Message>,
        _cmd: Command,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let answer_result = ctx.answer(Command::descriptions()).await;
        if let Err(e) = answer_result {
            return Err(Box::new(e));
        }
        Ok(())
    }
}
