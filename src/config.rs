use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Application configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// The list of canteens.
    pub canteens: Vec<Canteen>,

    /// Path to the backing file of the meow counter.
    pub meow_counter_file: PathBuf,

    /// Path to the backing file of the twd2 counter.
    pub twd2_counter_file: PathBuf,
}

/// Information about a canteen.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Canteen {
    /// The name of the canteen.
    pub name: String,

    /// The weight of the canteen.
    pub weight: u64,
}
