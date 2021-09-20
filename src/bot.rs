use std::error::Error;
use std::sync::Arc;

use teloxide::adaptors::AutoSend;
use teloxide::dispatching::UpdateWithCx;
use teloxide::requests::RequesterExt;
use teloxide::types::Message;
use teloxide::utils::command::BotCommand;
use teloxide::Bot as RawBot;

use crate::canteen::CanteenPicker;

/// `thufood` telegram bot.
pub struct Bot {
    token: String,
    picker: CanteenPicker,
}

impl Bot {
    /// Create a new `Bot` object from the given Telegram bot API token and canteen picker.
    pub fn new(token: String, picker: CanteenPicker) -> Self {
        Self { token, picker }
    }

    /// Run the bot.
    pub async fn run(self) {
        let bot = RawBot::new(self.token.clone()).auto_send();
        let picker = Arc::new(self.picker);
        teloxide::commands_repl(bot, "thufood", move |cx, cmd| {
            Self::handle_message(cx, cmd, picker.clone())
        }).await;
    }

    async fn handle_message(
        cx: UpdateWithCx<AutoSend<RawBot>, Message>,
        cmd: Command,
        picker: Arc<CanteenPicker>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match cmd {
            Command::Help => cx.answer(Command::descriptions()).await?,
            Command::Canteen => {
                let canteen = picker.pick();
                cx.answer(canteen.name.clone()).await?
            }
        };

        Ok(())
    }
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "支持的命令：")]
enum Command {
    #[command(description = "显示帮助信息")]
    Help,

    #[command(description = "随机选择一个餐厅")]
    Canteen,
}
