use serde::{Deserialize, Serialize};

/// Application configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub canteens: Vec<Canteen>,
}

/// Information about a canteen.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Canteen {
    /// The name of the canteen.
    pub name: String,

    /// The weight of the canteen.
    pub weight: u64,
}
