use std::sync::LazyLock;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Record {
    pub level: String,
    pub gates: u64,
    pub delay: u64,
    pub ticks: u64,
}

pub static DATA: LazyLock<Box<[Record]>> = LazyLock::new(|| {
    csv::Reader::from_path("data.csv")
        .expect("file `data.csv` is missing!")
        .deserialize()
        .flatten()
        .collect()
});

pub fn from_level(level: &str) -> impl Iterator<Item = &Record> {
    DATA.iter().filter(move |record| record.level == level)
}
