use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering as MemoryOrdering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use teloxide::adaptors::AutoSend;
use teloxide::dispatching::UpdateWithCx;
use teloxide::types::Message;
use teloxide::Bot;

use crate::commands::{Command, CommandHandler};
use crate::config::Config;

/// Handler for the `/feedmeow` command.
pub struct MeowCommandHandler {
    path: PathBuf,
    counter: AtomicU64,
    counter_write_lock: Mutex<()>,
}

impl MeowCommandHandler {
    fn load_meow_counter<P: AsRef<Path>>(path: P) -> u64 {
        match std::fs::read_to_string(path) {
            Ok(s) => match s.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse counter from file: {}", e);
                    0
                }
            },
            Err(e) => {
                log::warn!("Failed to read counter from file: {}", e);
                0
            }
        }
    }

    fn increase_meow_counter(&self) -> u64 {
        let ret = self.counter.fetch_add(1, MemoryOrdering::Relaxed);

        {
            let _lock = self.counter_write_lock.lock().unwrap();
            if let Err(e) = std::fs::write(&self.path, ret.to_string()) {
                log::warn!("Failed to write counter to file: {}", e);
            }
        }

        ret
    }
}

#[async_trait]
impl CommandHandler for MeowCommandHandler {
    fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let handler = Self {
            path: config.meow_counter_file.clone(),
            counter: AtomicU64::new(Self::load_meow_counter(&config.meow_counter_file)),
            counter_write_lock: Mutex::new(()),
        };
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
        let counter = self.increase_meow_counter();
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
