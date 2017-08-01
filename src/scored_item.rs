use std::cmp::Ordering;

#[derive(PartialEq,Debug)]
pub struct ScoredItem {
  pub item: u32,
  pub score: f64,
}

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
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(cmp_reverse(self, other))
  }
}