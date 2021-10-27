use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use rand::Rng;
use teloxide::adaptors::AutoSend;
use teloxide::dispatching::UpdateWithCx;
use teloxide::types::Message;
use teloxide::Bot;

use crate::commands::{Command, CommandHandler};
use crate::config::{Canteen, Config};

/// Handler of the `/canteen` command.
#[derive(Clone, Debug)]
pub struct CanteenCommandHandler {
    picker: CanteenPicker,
}

#[async_trait]
impl CommandHandler for CanteenCommandHandler {
    fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let handler = Self {
            picker: CanteenPicker::new(config.canteens.clone()),
        };
        Ok(handler)
    }

    fn accept(self: Arc<Self>, cmd: &Command) -> bool {
        match cmd {
            Command::Canteen => true,
            _ => false,
        }
    }

    async fn handle(
        self: Arc<Self>,
        ctx: UpdateWithCx<AutoSend<Bot>, Message>,
        _cmd: Command,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let canteen = self.picker.pick();
        ctx.answer(canteen.name.clone()).await?;
        Ok(())
    }
}

/// Randomly choose a canteen from a canteens list.
///
/// The random choice algorithm takes respect to the weights of each canteen.
#[derive(Clone, Debug)]
struct CanteenPicker {
    canteens: Vec<Canteen>,
    weight_sums: Vec<u64>,
    weight_sum: u64,
}

impl CanteenPicker {
    /// Create a new `CanteenPicker` object.
    ///
    /// This function panics if the given canteens list is empty.
    fn new<T, I>(canteens: T) -> Self
    where
        T: IntoIterator<Item = Canteen, IntoIter = I>,
        I: Iterator<Item = Canteen>,
    {
        let canteens: Vec<_> = canteens.into_iter().collect();
        assert!(!canteens.is_empty());

        let mut weight_sums = Vec::with_capacity(canteens.len());
        let mut weight_sum = 0u64;
        for i in &canteens {
            weight_sum += i.weight;
            weight_sums.push(weight_sum);
        }

        Self {
            canteens,
            weight_sums,
            weight_sum,
        }
    }

    /// Randomly choose a canteen.
    fn pick(&self) -> &Canteen {
        let sample = rand::thread_rng().gen_range(1..=self.weight_sum);
        let idx = match self.weight_sums.binary_search(&sample) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        &self.canteens[idx]
    }
}
