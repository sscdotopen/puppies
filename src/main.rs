extern crate csv;
extern crate rand;
extern crate fnv;
extern crate scoped_pool;

use std::collections::BinaryHeap;
use std::sync::Mutex;
use std::time::Instant;

use rand::Rng;
use fnv::{FnvHashMap, FnvHashSet};
use scoped_pool::Pool;

mod loglikelihoodratio;
mod scored_item;

use scored_item::ScoredItem;



fn main() {

  let pool_size: usize = std::env::args().nth(1).unwrap().parse().unwrap();

  let pool = Pool::new(pool_size);

  let file =
    "/home/ssc/Entwicklung/projects/incremental-cooccurrences/src/main/resources/ml1m-shuffled.csv";

  const NUM_USERS: usize = 9746;
  const NUM_ITEMS: usize = 6040;

  const F_MAX: u32 = 500;
  const K_MAX: u32 = 500;
  const K: usize = 10;
  const BATCH_SIZE: usize = 10000;

  let mut user_non_sampled_interaction_counts: [u32; NUM_USERS] = [0; NUM_USERS];
  let mut user_interaction_counts: [u32; NUM_USERS] = [0; NUM_USERS];
  let mut item_interaction_counts: [u32; NUM_ITEMS] = [0; NUM_ITEMS];

  let mut samples_of_a: Vec<Vec<u32>> = std::iter::repeat(Vec::with_capacity(10))
      .take(NUM_USERS)
      .collect::<Vec<Vec<u32>>>();

  let mut c: Vec<FnvHashMap<u32,u16>> = Vec::with_capacity(NUM_ITEMS);
  let mut indicators: Vec<Mutex<BinaryHeap<ScoredItem>>> = Vec::with_capacity(NUM_ITEMS);

  for _ in 0..NUM_ITEMS {
    c.push(FnvHashMap::with_capacity_and_hasher(10, Default::default()));
    indicators.push(Mutex::new(BinaryHeap::with_capacity(K)));
  }

  let mut row_sums_of_c: [u32; NUM_ITEMS] = [0; NUM_ITEMS];

  let mut num_interactions_observed: u64 = 0;
  let mut num_cooccurrences_observed: u64 = 0;

  let mut rng = rand::XorShiftRng::new_unseeded();

  let batches = read_file_into_batches(&file, BATCH_SIZE);


  for batch in batches.iter() {

    let batch_start = Instant::now();

    let mut items_to_rescore = FnvHashSet::default();

    for &(user, item) in batch.iter() {

      user_non_sampled_interaction_counts[user as usize] += 1;

      num_interactions_observed += 1;

      if item_interaction_counts[item as usize] < F_MAX {
        let mut user_history = samples_of_a.get_mut(user as usize).unwrap();
        let num_items_in_user_history = user_history.len();

        if user_interaction_counts[user as usize] < K_MAX {
          for other_item in user_history.iter() {

            *c[item as usize].entry(*other_item).or_insert(0) += 1;
            *c[*other_item as usize].entry(item).or_insert(0) += 1;

            row_sums_of_c[*other_item as usize] += 1;
          }

          row_sums_of_c[item as usize] += num_items_in_user_history as u32;
          num_cooccurrences_observed += 2 * num_items_in_user_history as u64;

          user_history.push(item);

          user_interaction_counts[user as usize] += 1;
          item_interaction_counts[item as usize] += 1;

          items_to_rescore.insert(item);

        } else {

          let num_interactions_seen_by_user = user_non_sampled_interaction_counts[user as usize];
          let k: usize = rng.gen_range(0, num_interactions_seen_by_user as usize);

          if k < num_items_in_user_history {
            let previous_item = user_history[k];

            for (n, other_item) in user_history.iter().enumerate() {
              if n != k {

                *c[item as usize].entry(*other_item).or_insert(0) += 1;
                *c[*other_item as usize].entry(item).or_insert(0) += 1;

                *c[previous_item as usize].entry(*other_item).or_insert(0) -= 1;
                *c[*other_item as usize].entry(previous_item).or_insert(0) -= 1;
              }
            }

            row_sums_of_c[item as usize] += num_items_in_user_history as u32 - 1;
            row_sums_of_c[previous_item as usize] -= num_items_in_user_history as u32 - 1;

            user_history[k] = item;

            item_interaction_counts[item as usize] += 1;
            item_interaction_counts[previous_item as usize] -= 1;

            items_to_rescore.insert(item);
          }
        }
      }
    }

    pool.scoped(|scope| {
      for item in items_to_rescore.iter() {

        let row = &c[*item as usize];
        let indicators_for_item = &indicators[*item as usize];

        scope.execute(move|| {
          rescore(*item, row, &row_sums_of_c, &num_cooccurrences_observed, indicators_for_item, K)
        });
      }
    });


    let millis = (batch_start.elapsed().as_secs() * 1_000) +
        (batch_start.elapsed().subsec_nanos() / 1_000_000) as u64;
    println!("{} ({}ms for last batch, {} items rescored)", num_interactions_observed, millis,
        items_to_rescore.len());
  }

}

fn read_file_into_batches(file: &str, batch_size: usize) -> Vec<Vec<(u32, u32)>> {

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


fn rescore(item: u32, cooccurrence_counts: &FnvHashMap<u32,u16>, row_sums_of_c: &[u32],
  num_cooccurrences_observed: &u64, indicators: &Mutex<BinaryHeap<ScoredItem>>, k: usize) {

  let mut indicators_for_item = indicators.lock().unwrap();
  indicators_for_item.clear();

  for (other_item, num_cooccurrences) in cooccurrence_counts.iter() {

    let k11 = *num_cooccurrences as u64;
    let k12 = row_sums_of_c[item as usize] as u64 - k11;
    let k21 = row_sums_of_c[*other_item as usize] as u64 - k11;
    let k22 = num_cooccurrences_observed + k11 - k12 - k21;

    let llr_score = loglikelihoodratio::log_likelihood_ratio(k11, k12, k21, k22);

    let scored_item = ScoredItem { item: *other_item, score: llr_score };

    if indicators_for_item.len() < k {
      indicators_for_item.push(scored_item);
    } else {
      let mut top = indicators_for_item.peek_mut().unwrap();
      if scored_item < *top {
        *top = scored_item;
      }
    }

  }
}