struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
    let mut ans: i64 = 0;
    let mut buf: Vec<i64> = Vec::new();
    values.iter().for_each(|row| {
      row.iter().for_each(|&v| {
        buf.push(v as i64);
      });
    });
    buf.sort_unstable();
    buf.iter().enumerate().for_each(|(idx, v)| {
      ans += v * (idx as i64 + 1);
    });
    ans
  }

  pub fn max_spending2(values: Vec<Vec<i32>>) -> i64 {
    let mut ans: i64 = 0;
    let mut idxs: Vec<i32> = Vec::new();

    let mut bh: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();
    (0..values.len()).for_each(|idx| {
      bh.push(Reverse((
        values[idx][values[idx].len() - 1] as i64,
        idx,
        values[idx].len() - 1,
      )));
    });

    let mut d: i64 = 1;
    while bh.len() > 0 {
      let (v, idx1, idx2) = bh.pop().unwrap().0;
      ans += v * d;
      if idx2 > 0 {
        bh.push(Reverse((values[idx1][idx2 - 1] as i64, idx1, idx2 - 1)));
      }
      d += 1;
    }
    ans
  }
}
