use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rand::Rng;

/// A canteen.
#[derive(Clone, Debug)]
pub struct Canteen {
    /// Name of the canteen.
    pub name: String,

    /// Weight of the canteen.
    ///
    /// The higher the weight is, the more likely the canteen is been chosen.
    pub weight: u32,
}

impl Display for Canteen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

/// Error occurred when loading canteens list from a file.
#[derive(Debug)]
pub enum LoadCanteensError {
    Io(std::io::Error),
    Format(String),
    NoCanteens,
}

impl From<std::io::Error> for LoadCanteensError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl Display for LoadCanteensError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => f.write_fmt(format_args!("io error: {}", err)),
            Self::Format(message) => f.write_str(message),
            Self::NoCanteens => f.write_str("no canteens loaded"),
        }
    }
}

impl Error for LoadCanteensError {}

/// Load canteens list from a file.
///
/// Each line of the file content should be in the following form: `<name>,<weight>` where `<name>`
/// is the name of a canteen and `<weight>` is the weight of the canteen.
pub fn load_canteens_from_file<P>(file_path: P) -> Result<Vec<Canteen>, LoadCanteensError>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    let file_reader = BufReader::new(file);

    let mut canteens = Vec::new();
    for ln in file_reader.lines() {
        let ln = ln?;
        let ln_parts: Vec<_> = ln.split(',').collect();
        if ln_parts.len() != 2 {
            return Err(LoadCanteensError::Format(format!(
                "invalid format: \"{}\"",
                ln
            )));
        }

        let canteen_weight = match ln_parts[1].parse::<u32>() {
            Ok(weight) => weight,
            Err(_) => {
                return Err(LoadCanteensError::Format(format!(
                    "invalid canteen weight: \"{}\"",
                    ln_parts[1]
                )));
            }
        };

        canteens.push(Canteen {
            name: String::from(ln_parts[0]),
            weight: canteen_weight,
        });
    }

    if canteens.is_empty() {
        return Err(LoadCanteensError::NoCanteens);
    }

    Ok(canteens)
}

/// Randomly choose a canteen from a canteens list.
///
/// The random choice algorithm takes respect to the weights of each canteen.
#[derive(Clone, Debug)]
pub struct CanteenPicker {
    canteens: Vec<Canteen>,
    weight_sums: Vec<u32>,
    weight_sum: u32,
}

impl CanteenPicker {
    /// Create a new `CanteenPicker` object.
    ///
    /// This function panics if the given canteens list is empty.
    pub fn new<T, I>(canteens: T) -> Self
    where
        T: IntoIterator<Item = Canteen, IntoIter = I>,
        I: Iterator<Item = Canteen>,
    {
        let canteens: Vec<_> = canteens.into_iter().collect();
        assert!(!canteens.is_empty());

        let mut weight_sums = Vec::with_capacity(canteens.len());
        let mut weight_sum = 0u32;
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
    pub fn pick(&self) -> &Canteen {
        let sample = rand::thread_rng().gen_range(1..=self.weight_sum);
        let idx = match self.weight_sums.binary_search(&sample) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        &self.canteens[idx]
    }
}
