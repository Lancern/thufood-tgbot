use std::error::Error;
use std::sync::Arc;

use teloxide::adaptors::AutoSend;
use teloxide::dispatching::UpdateWithCx;
use teloxide::requests::{Requester, RequesterExt};
use teloxide::types::{BotCommand as BotCommandDescriptor, Message};
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
    pub async fn run(self) {
        let bot = RawBot::new(self.token.clone()).auto_send();

        // Register all the commands provided by the bot.
        match bot.set_my_commands(ALL_COMMANDS.clone()).await {
            Ok(_) => (),
            Err(e) => {
                log::warn!("cannot set commands: {}", e);
            }
        }

        let picker = Arc::new(self.picker);
        teloxide::commands_repl(bot, self.name, move |cx, cmd| {
            Self::handle_message(cx, cmd, picker.clone())
        })
        .await;
    }

    async fn handle_message(
        cx: UpdateWithCx<AutoSend<RawBot>, Message>,
        cmd: Command,
        picker: Arc<CanteenPicker>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match cmd {
            Command::Start | Command::Help => {
                cx.answer(Command::descriptions()).await?;
            }
            Command::Canteen => {
                let canteen = picker.pick();
                cx.answer(canteen.name.clone()).await?;
            }
            Command::Milktea => {
                if let Some(user) = crate::utils::get_message_sender(&cx.update) {
                    let user_name = crate::utils::get_user_display_name(user);
                    loop {
                        if let Some(reply_msg) = crate::utils::get_replied_message(&cx.update) {
                            if let Some(target_user) = crate::utils::get_message_sender(reply_msg) {
                                let target_user_name =
                                    crate::utils::get_user_display_name(target_user);
                                cx.answer(format!(
                                    "{} 给 {} 倒了一杯奶茶！🧋",
                                    user_name, target_user_name
                                ))
                                .await?;
                                break;
                            }
                        }
                        cx.answer(format!("给 {} 倒一杯奶茶！🧋", user_name))
                            .await?;
                        break;
                    }
                }
            }
        };

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
