use std::str::FromStr;

use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Arguments {
    #[options(default = "most-used")]
    pub algorithm: Algorithm,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Algorithm {
    SimpleAverage,
    MostUsed,
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "simple-average" => Ok(Algorithm::SimpleAverage),
            "most-used" => Ok(Algorithm::MostUsed),
            _ => Err(format!("Unknown algorithm: {}", s)),
        }
    }
}
