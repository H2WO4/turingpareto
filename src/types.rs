use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Record {
    pub level: String,
    pub gates: u64,
    pub delay: u64,
    pub ticks: u64,
}

impl Record {
    pub fn sum2(&self) -> u64 {
        self.gates + self.delay
    }

    pub fn sum3(&self) -> u64 {
        self.gates + self.delay + self.ticks
    }

    pub fn product2(&self) -> u64 {
        self.gates * self.delay
    }

    pub fn product3(&self) -> u64 {
        self.gates * self.delay * self.ticks
    }
}
