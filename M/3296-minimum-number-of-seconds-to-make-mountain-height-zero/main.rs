use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
  pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
    let mut min_heap: BinaryHeap<Reverse<(i64, usize, i64)>> = BinaryHeap::new();
    let mut mh = mountain_height;
    let mut ans: i64 = 0;
    worker_times.iter().enumerate().for_each(|(idx, v)| {
      min_heap.push(Reverse((*v as i64, idx, 1)));
    });

    while mh > 0 {
      let (sec, idx, times) = min_heap.pop().unwrap().0;
      let next_sec = sec + worker_times[idx] as i64 * (times + 1);
      ans = ans.max(sec);
      min_heap.push(Reverse((next_sec, idx, times + 1)));
      mh -= 1;
    }
    ans
  }
}
