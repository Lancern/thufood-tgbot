use std::error::Error;
use std::sync::Arc;

use teloxide::adaptors::AutoSend;
use teloxide::dispatching::UpdateWithCx;
use teloxide::requests::{Requester, RequesterExt};
use teloxide::types::{BotCommand as BotCommandDescriptor, Message, User};
use teloxide::utils::command::BotCommand;
use teloxide::Bot as RawBot;

use crate::canteen::CanteenPicker;

/// `thufood` telegram bot.
pub struct Bot {
    token: String,
    name: String,
    picker: CanteenPicker,
}

impl Bot {
    /// Create a new `Bot` object from the given Telegram bot API token, bot name and canteen picker.
    pub fn new(token: String, name: String, picker: CanteenPicker) -> Self {
        Self {
            token,
            name,
            picker,
        }
    }

    /// Run the bot.
    pub async fn run(self: Arc<Self>) {
        let bot = RawBot::new(self.token.clone()).auto_send();

        // Register all the commands provided by the bot.
        match bot.set_my_commands(ALL_COMMANDS.clone()).await {
            Ok(_) => (),
            Err(e) => {
                log::warn!("cannot set commands: {}", e);
            }
        }

        let self_share = self.clone();
        teloxide::commands_repl(bot, self.name.clone(), move |cx, cmd| {
            Self::handle_message(self_share.clone(), cx, cmd)
        })
        .await;
    }

    async fn handle_message(
        self: Arc<Self>,
        cx: UpdateWithCx<AutoSend<RawBot>, Message>,
        cmd: Command,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match cmd {
            Command::Start | Command::Help => {
                cx.answer(Command::descriptions()).await?;
            }
            Command::Canteen => {
                let canteen = self.picker.pick();
                cx.answer(canteen.name.clone()).await?;
            }
            Command::Milktea => {
                Self::give_drinks(cx, "奶茶", "🧋").await?;
            }
        };

        Ok(())
    }

    async fn give_drinks(
        cx: UpdateWithCx<AutoSend<RawBot>, Message>,
        drink_name: &str,
        drink_emoji: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let from = match crate::utils::get_message_sender(&cx.update) {
            Some(user) => user,
            None => return Ok(()),
        };
        let to = crate::utils::get_replied_message(&cx.update).and_then(crate::utils::get_message_sender);
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
        let from_name = crate::utils::get_user_display_name(from);
        let mut to_name = match to {
            Some(user) => crate::utils::get_user_display_name(user),
            None => String::from("自己"),
        };
        if from_name == to_name {
            to_name = String::from("自己");
        }

        format!(
            "{} 给 {} 倒了一杯{}！{}",
            from_name, to_name, drink_name, drink_emoji
        )
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
    ];
}
