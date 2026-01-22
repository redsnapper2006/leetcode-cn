use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
  pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    let mut buf: Vec<(i64, usize, usize)> = vec![];
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    let mut cnt: i32 = 0;
    for i in 0..nums.len() {
      buf.push((nums[i] as i64, i, i));
      if i + 1 < nums.len() {
        heap.push(Reverse((nums[i] as i64 + nums[i + 1] as i64, i)));
        if nums[i] > nums[i + 1] {
          cnt += 1;
        }
      }
    }

    let mut times: i32 = 0;
    while heap.len() > 0 && cnt > 0 {
      let (sum, idx) = heap.pop().unwrap().0;
      let s1 = buf[idx].1;
      let e1 = buf[idx].2;
      if e1 == buf.len() - 1
        || !(s1 == idx && buf[e1].1 == s1 || e1 == idx && buf[s1].2 == idx)
        || buf[s1].0 + buf[buf[e1 + 1].2].0 != sum
      {
        continue;
      }

      let e2 = buf[e1 + 1].2;
      let v = buf[s1].0 + buf[e2].0;
      times += 1;

      if s1 > 0 && buf[s1 - 1].0 > buf[s1].0 {
        cnt -= 1;
      }
      if e2 < buf.len() - 1 && buf[e2].0 > buf[e2 + 1].0 {
        cnt -= 1;
      }
      if buf[s1].0 > buf[e2].0 {
        cnt -= 1;
      }

      buf[s1].0 = v;
      buf[e2].0 = v;
      buf[e2].1 = s1;
      buf[s1].2 = e2;
      if s1 > 0 {
        heap.push(Reverse((buf[s1 - 1].0 + v, s1 - 1)));
        if buf[s1 - 1].0 > buf[s1].0 {
          cnt += 1;
        }
      }
      if e2 < buf.len() - 1 {
        heap.push(Reverse((buf[e2 + 1].0 + v, e2)));
        if buf[e2].0 > buf[e2 + 1].0 {
          cnt += 1;
        }
      }
    }
    times
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::minimum_pair_removal(vec![3, -3, -2, 2, 2, 0, 3, 0, 1, 0, 3, -2])
  );
}
