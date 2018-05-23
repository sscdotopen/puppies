extern crate csv;

use std::time::Duration;

pub fn to_millis(duration: Duration) -> u64 {
     (duration.as_secs() * 1_000) + (duration.subsec_nanos() / 1_000_000) as u64
}

pub fn read_file_into_batches(file: &str, batch_size: usize) -> Vec<Vec<(u32, u32)>> {

    let mut csv_reader = csv::Reader::from_file(file).unwrap().has_headers(false);

    let mut batches = vec![];
    let mut current_batch: Vec<(u32, u32)> = Vec::with_capacity(batch_size);

    for record in csv_reader.decode() {
        let (user, item): (u32, u32) = record.unwrap();

        if current_batch.len() < batch_size {
            current_batch.push((user, item));
        } else {
            batches.push(current_batch.clone());
            current_batch.clear();
        }
    }

    if !current_batch.is_empty() {
        batches.push(current_batch.clone());
    }

    batches
}