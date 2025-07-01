use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
  pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut k = k as usize;
    let mut bb = nums2
      .into_iter()
      .zip(nums1.into_iter())
      .map(|(x, y)| (x as i64, y as i64))
      .collect::<Vec<(i64, i64)>>();
    bb.sort_by(|x, y| {
      let x1 = &x.0;
      let y1 = &y.0;
      let x2 = &x.1;
      let y2 = &y.1;
      if x1 == y1 {
        return y2.cmp(x2);
      }
      y1.cmp(x1)
    });

    let mut bh: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut sum: i64 = 0;
    (0..k).for_each(|idx| {
      bh.push(Reverse(bb[idx].1));
      sum += bb[idx].1;
    });

    let mut ans: i64 = sum * bb[k - 1].0;

    for ii in k..bb.len() {
      sum -= bh.pop().unwrap().0;
      sum += bb[ii].1;
      bh.push(Reverse(bb[ii].1));
      ans = ans.max(sum * bb[ii].0);
    }

    ans
  }
}
