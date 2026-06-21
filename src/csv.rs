use std::sync::LazyLock;

use crate::types::Record;

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
