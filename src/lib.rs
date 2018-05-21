extern crate rand;
extern crate fnv;
extern crate scoped_pool;

use std::collections::BinaryHeap;
use std::sync::Mutex;
use std::time::Instant;

use rand::Rng;
use fnv::{FnvHashMap, FnvHashSet};
use scoped_pool::Pool;

pub mod llr;
mod utils;


// cargo run --release /home/ssc/Entwicklung/projects/incremental-cooccurrences/src/main/resources/ml1m-shuffled.csv 9746 6040 8
// cargo run --release /home/ssc/Entwicklung/projects/incremental-cooccurrences/src/main/resources/dblp-shuffled.csv 1314051 1314051 8
// cargo run --release /home/ssc/Entwicklung/datasets/twitterhashtags/twitter.csv 8094909 3070055 8

pub fn incremental_indicators(file: String, num_users: usize, num_items: usize, pool_size: usize) {

  let pool = Pool::new(pool_size);

  const F_MAX: u32 = 500;
  const K_MAX: u32 = 500;
  const K: usize = 10;
  const BATCH_SIZE: usize = 10000;

  // larger of both values needs to be added
  const MAX_COOCCURRENCES: usize = (F_MAX * K_MAX + K_MAX) as usize;
  let pre_computed_logarithms: Vec<f64> = llr::logarithms_table(MAX_COOCCURRENCES);


  let mut user_non_sampled_interaction_counts: Vec<u32> = utils::int_vec_of_size(num_users, 0);
  let mut user_interaction_counts: Vec<u32> = utils::int_vec_of_size(num_users, 0);
  let mut item_interaction_counts: Vec<u32> = utils::int_vec_of_size(num_items, 0);

  let mut samples_of_a: Vec<Vec<u32>> = std::iter::repeat(Vec::with_capacity(10))
      .take(num_users)
      .collect::<Vec<Vec<u32>>>();

  let mut c: Vec<FnvHashMap<u32,u16>> = Vec::with_capacity(num_items);
  let mut indicators: Vec<Mutex<BinaryHeap<llr::ScoredItem>>> = Vec::with_capacity(num_items);

  for _ in 0..num_items {
    c.push(FnvHashMap::with_capacity_and_hasher(10, Default::default()));
    indicators.push(Mutex::new(BinaryHeap::with_capacity(K)));
  }

  let mut row_sums_of_c: Vec<u32> = utils::int_vec_of_size(num_items, 0);

  let mut num_interactions_observed: u64 = 0;
  let mut num_cooccurrences_observed: u64 = 0;

  let mut rng = rand::XorShiftRng::new_unseeded();

  let batches = utils::read_file_into_batches(&file, BATCH_SIZE);

  println!("{} batches to process", batches.len());

  let mut duration_for_all_batches: u64 = 0;
  let mut num_items_rescored_in_all_batches: u64 = 0;

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
            items_to_rescore.insert(*other_item);
          }

          row_sums_of_c[item as usize] += num_items_in_user_history as u32;
          num_cooccurrences_observed += 2 * num_items_in_user_history as u64;

          user_history.push(item);

          user_interaction_counts[user as usize] += 1;
          item_interaction_counts[item as usize] += 1;

          items_to_rescore.insert(item);

        } else {

          let num_interactions_seen_by_user =
            user_non_sampled_interaction_counts[user as usize];
          let k: usize = rng.gen_range(0, num_interactions_seen_by_user as usize);

          if k < num_items_in_user_history {
            let previous_item = user_history[k];

            for (n, other_item) in user_history.iter().enumerate() {
              if n != k {

                *c[item as usize].entry(*other_item).or_insert(0) += 1;
                *c[*other_item as usize].entry(item).or_insert(0) += 1;

                *c[previous_item as usize].entry(*other_item).or_insert(0) -= 1;
                *c[*other_item as usize].entry(previous_item).or_insert(0) -= 1;

                items_to_rescore.insert(*other_item);
              }
            }

            row_sums_of_c[item as usize] += num_items_in_user_history as u32 - 1;
            row_sums_of_c[previous_item as usize] -= num_items_in_user_history as u32 - 1;

            user_history[k] = item;

            item_interaction_counts[item as usize] += 1;
            item_interaction_counts[previous_item as usize] -= 1;

            items_to_rescore.insert(previous_item);
            items_to_rescore.insert(item);
          }
        }
      }
    }

    pool.scoped(|scope| {
      for item in items_to_rescore.iter() {

        let row = &c[*item as usize];
        let indicators_for_item = &indicators[*item as usize];
        let reference_to_row_sums_of_c = &row_sums_of_c;
        let reference_to_pre_computed_logarithms = &pre_computed_logarithms;

        scope.execute(move|| {
          rescore(*item, row, reference_to_row_sums_of_c, &num_cooccurrences_observed,
                  indicators_for_item, K, reference_to_pre_computed_logarithms)
        });
      }
    });

    let duration_for_batch = utils::to_millis(batch_start.elapsed());
    println!("{}, {}ms for last batch, {} items rescored", num_interactions_observed,
             duration_for_batch, items_to_rescore.len());

    duration_for_all_batches += duration_for_batch;
    num_items_rescored_in_all_batches += items_to_rescore.len() as u64;
  }

  println!("Overall {}ms, {}ms avg per batch, {} items rescored, {} num cooccurrences observed",
           duration_for_all_batches, duration_for_all_batches / batches.len() as u64,
           num_items_rescored_in_all_batches, num_cooccurrences_observed)
}


fn rescore(item: u32, cooccurrence_counts: &FnvHashMap<u32,u16>, row_sums_of_c: &[u32],
           num_cooccurrences_observed: &u64, indicators: &Mutex<BinaryHeap<llr::ScoredItem>>,
           k: usize, logarithms_table: &Vec<f64>) {

  let mut indicators_for_item = indicators.lock().unwrap();
  indicators_for_item.clear();

  for (other_item, num_cooccurrences) in cooccurrence_counts.iter() {

    let k11 = *num_cooccurrences as u64;
    let k12 = row_sums_of_c[item as usize] as u64 - k11;
    let k21 = row_sums_of_c[*other_item as usize] as u64 - k11;
    let k22 = num_cooccurrences_observed + k11 - k12 - k21;

    let llr_score = llr::log_likelihood_ratio(k11, k12, k21, k22, logarithms_table);

    let scored_item = llr::ScoredItem { item: *other_item, score: llr_score };

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