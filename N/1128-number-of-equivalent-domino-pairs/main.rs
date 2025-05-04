struct Solution {}

impl Solution {
  pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    dominoes
      .iter()
      .fold(vec![0; 100], |mut aggr, domino| {
        let mut offset = domino[0] * 10 + domino[1];
        if domino[0] > domino[1] {
          offset = domino[1] * 10 + domino[0];
        }
        aggr[offset as usize] += 1;
        aggr
      })
      .iter()
      .fold(0, |aggr, v| aggr + v * (v - 1) / 2)
  }
}
