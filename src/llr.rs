use std::cmp::Ordering;

#[derive(PartialEq,Debug)]
pub struct ScoredItem {
  pub item: u32,
  pub score: f64,
}

//max heap
fn cmp_reverse(scored_item_a: &ScoredItem, scored_item_b: &ScoredItem) -> Ordering {
  match scored_item_a.score.partial_cmp(&scored_item_b.score) {
    Some(Ordering::Less) => Ordering::Greater,
    Some(Ordering::Greater) => Ordering::Less,
    Some(Ordering::Equal) => Ordering::Equal,
    None => Ordering::Equal
  }
}

impl Eq for ScoredItem {}

impl Ord for ScoredItem {
  fn cmp(&self, other: &Self) -> Ordering { cmp_reverse(self, other) }
}

impl PartialOrd for ScoredItem {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(cmp_reverse(self, other)) }
}

/* https://github.com/apache/mahout/blob/08e02602e947ff945b9bd73ab5f0b45863df3e53/math/src/main/java/org/apache/mahout/math/stats/LogLikelihood.java  */
#[inline(always)]
pub fn log_likelihood_ratio(k11: u64, k12: u64, k21: u64, k22: u64) -> f64 {

  /* Thank you Frank - https://www.reddit.com/r/rust/comments/6qmnbo/why_is_my_scala_program_twice_as_fast_as_my_rust/dl0x1bj/ */

  let xlx_all = x_log_x(k11 + k12 + k21 + k22);

  let row_entropy = xlx_all - x_log_x(k11 + k12) - x_log_x(k21 + k22);
  let column_entropy = xlx_all - x_log_x(k11 + k21) - x_log_x(k12 + k22);

  let matrix_entropy = xlx_all - x_log_x(k11) - x_log_x(k12) -  x_log_x(k21) - x_log_x(k22);

  if row_entropy + column_entropy < matrix_entropy {
    // round off error
    0.0
  } else {
    2.0 * (row_entropy + column_entropy - matrix_entropy)
  }
}

#[inline(always)]
fn x_log_x(x: u64) -> f64 {
  if x == 0 { 0.0 } else { (x as f64) * (x as f64).ln() }
}



#[cfg(test)]
mod tests {

  use std::collections::BinaryHeap;
  use llr;

  # [test]
  fn llr() {
    // Some cases from Ted's paper http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.14.5962
    const EPS: f64 = 0.1;

    assert!(close_enough_to(llr::log_likelihood_ratio(110, 2442, 111, 29114), 270.72, EPS));
    assert!(close_enough_to(llr::log_likelihood_ratio(29, 13, 123, 31612), 263.90, EPS));
    assert!(close_enough_to(llr::log_likelihood_ratio(9, 12, 429, 31327), 48.94, EPS));
  }

  fn close_enough_to(value: f64, expected: f64, eps: f64) -> bool {
    (value - expected).abs() < eps
  }

  #[test]
  fn topk() {

    const K: usize = 3;

    let items = [
      llr::ScoredItem { item: 1, score: 0.5 },
      llr::ScoredItem { item: 2, score: 1.5 },
      llr::ScoredItem { item: 3, score: 0.3 },
      llr::ScoredItem { item: 4, score: 3.5 },
      llr::ScoredItem { item: 5, score: 2.5 },
    ];

    let mut heap = BinaryHeap::with_capacity(K);

    for scored_item in items.iter() {
      if heap.len() < K {
        heap.push(scored_item);
      } else {
        let mut top = heap.peek_mut().unwrap();
        if scored_item < *top {
          *top = scored_item;
        }
      }
    }

    let top_k = heap.into_sorted_vec();

    assert!(top_k.len() == 3);

    assert!(top_k[0].item == 4);
    assert!(top_k[0].score == 3.5);

    assert!(top_k[1].item == 5);
    assert!(top_k[1].score == 2.5);

    assert!(top_k[2].item == 2);
    assert!(top_k[2].score == 1.5);
  }
}