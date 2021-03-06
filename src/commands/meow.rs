use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use teloxide::adaptors::AutoSend;
use teloxide::dispatching::UpdateWithCx;
use teloxide::types::Message;
use teloxide::Bot;

use crate::commands::{Command, CommandHandler};
use crate::config::Config;
use crate::services::counter::CounterService;

/// Handler for the `/feedmeow` command.
pub struct MeowCommandHandler {
    service: CounterService,
}

#[async_trait]
impl CommandHandler for MeowCommandHandler {
    fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let service = CounterService::new(&config.meow_counter_file);
        let handler = Self { service };
        Ok(handler)
    }

    fn accept(self: Arc<Self>, cmd: &Command) -> bool {
        match cmd {
            Command::FeedMeow | Command::FeedMeowWd40 => true,
            _ => false,
        }
    }

    async fn handle(
        self: Arc<Self>,
        ctx: UpdateWithCx<AutoSend<Bot>, Message>,
        cmd: Command,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let counter = self.service.increase();
        let msg = match cmd {
            Command::FeedMeow => format!("Meow~~~\n猫咪投喂计数器：{}\n呼呼喵喵zzz", counter),
            Command::FeedMeowWd40 => {
                format!("Meow~~~\n猫咪投喂计数器：{}\n精神喵喵！！！", counter)
            }
            _ => unreachable!(),
        };
        ctx.answer(msg).await?;
        Ok(())
    }
}
