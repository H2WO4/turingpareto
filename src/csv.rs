use std::sync::LazyLock;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
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

pub fn from_level(level: &str) -> Box<[&Record]> {
    DATA.iter().filter(|record| record.level == level).collect()
}
