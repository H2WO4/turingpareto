use std::sync::LazyLock;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Record {
    level: String,
    gates: u64,
    delay: u64,
    ticks: u64,
}

pub static DATA: LazyLock<Box<[Record]>> = LazyLock::new(|| {
    csv::Reader::from_path("data.csv")
        .expect("file `data.csv` is missing!")
        .deserialize()
        .flatten()
        .collect()
});

pub fn from_level(level: &str) -> Box<[&Record]> {
    DATA.iter().filter(|record| record.level == level).collect()
}
