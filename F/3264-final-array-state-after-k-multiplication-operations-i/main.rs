use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let mut h: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    nums.iter().enumerate().for_each(|(idx, &v)| {
      h.push(Reverse((v, idx)));
    });
    (0..k).for_each(|_| {
      let (v, idx) = h.pop().unwrap().0;
      h.push(Reverse((v * multiplier, idx)));
    });

    let mut ans: Vec<i32> = vec![0; nums.len()];
    while h.len() > 0 {
      let (v, idx) = h.pop().unwrap().0;
      ans[idx] = v;
    }

    ans
  }
}
