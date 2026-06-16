use crate::csv::Record;

pub fn filter<'a>(records: impl Iterator<Item = &'a Record>) -> Vec<&'a Record> {
    records.fold(vec![], move |mut acc: Vec<&Record>, record| {
        for r in &acc {
            if r.gates <= record.gates && r.delay <= record.delay {
                return acc;
            }
        }
        acc.push(record);
        acc
    })
}
