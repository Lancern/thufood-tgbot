use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use teloxide::adaptors::AutoSend;
use teloxide::dispatching::UpdateWithCx;
use teloxide::types::{Message, User};
use teloxide::Bot;

use crate::commands::{Command, CommandHandler};
use crate::config::Config;

/// Handler of the `/milktea`, the `/cappuccino` and the `producedrink` commands.
#[derive(Clone, Debug, Default)]
pub struct DrinkCommandHandler;

impl DrinkCommandHandler {
    async fn give_drinks(
        cx: UpdateWithCx<AutoSend<Bot>, Message>,
        drink_name: &str,
        drink_emoji: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let from = match crate::utils::get_message_sender(&cx.update) {
            Some(user) => user,
            None => return Ok(()),
        };
        let to = crate::utils::get_replied_message(&cx.update)
            .and_then(crate::utils::get_message_sender);
        let response = Self::format_give_drink_message(from, to, drink_name, drink_emoji);
        cx.answer(response).await?;
        Ok(())
    }

    fn format_give_drink_message(
        from: &User,
        to: Option<&User>,
        drink_name: &str,
        drink_emoji: &str,
    ) -> String {
        let mut to = to;
        if let Some(u) = &to {
            if from.id == u.id {
                to = None;
            }
        }

        let from_name = crate::utils::get_user_display_name(from);

        if let Some(u) = &to {
            let to_name = crate::utils::get_user_display_name(u);
            format!(
                "{} 给 {} 倒了一杯{}！{}",
                from_name, to_name, drink_name, drink_emoji
            )
        } else {
            format!(
                "{} 给自己倒了一杯{}！{}",
                from_name, drink_name, drink_emoji
            )
        }
    }
}

#[async_trait]
impl CommandHandler for DrinkCommandHandler {
    fn new(_config: &Config) -> Result<Self, Box<dyn Error>> {
        Ok(Self)
    }

    fn accept(self: Arc<Self>, cmd: &Command) -> bool {
        match cmd {
            Command::Milktea | Command::Cappuccino | Command::ProduceDrink { .. } => true,
            _ => false,
        }
    }

    async fn handle(
        self: Arc<Self>,
        ctx: UpdateWithCx<AutoSend<Bot>, Message>,
        cmd: Command,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match cmd {
            Command::Milktea => Self::give_drinks(ctx, "奶茶", "🧋").await,
            Command::Cappuccino => Self::give_drinks(ctx, "卡布奇诺", "☕️").await,
            Command::ProduceDrink { drink_name } => Self::give_drinks(ctx, &drink_name, "").await,
            _ => unreachable!(),
        }
    }
}
