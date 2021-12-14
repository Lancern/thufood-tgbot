use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use teloxide::adaptors::AutoSend;
use teloxide::dispatching::UpdateWithCx;
use teloxide::prelude::Message;
use teloxide::Bot;

use crate::commands::{Command, CommandHandler};
use crate::services::counter::CounterService;
use crate::Config;

/// Handler for the `/feedtwd2` command.
pub struct Twd2CommandHandler {
    service: CounterService,
}

#[async_trait]
impl CommandHandler for Twd2CommandHandler {
    fn new(config: &Config) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let service = CounterService::new(&config.twd2_counter_file);
        let handler = Self { service };
        Ok(handler)
    }

    fn accept(self: Arc<Self>, cmd: &Command) -> bool {
        match cmd {
            Command::FeedTwd2 => true,
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
            Command::FeedMeow => {
                format!("二老师投喂计数器：{}\n向二老师投喂酥酥万呆粒一枚~", counter)
            }
            _ => unreachable!(),
        };
        ctx.answer(msg).await?;
        Ok(())
    }
}
