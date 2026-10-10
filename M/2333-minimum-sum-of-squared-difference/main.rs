use std::collections::{BinaryHeap, HashMap};

impl Solution {
  pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
    let mut map: HashMap<i64, i64> = HashMap::new();
    let mut heap: BinaryHeap<i64> = BinaryHeap::new();
    for i in 0..nums1.len() {
      *map.entry((nums1[i] as i64 - nums2[i] as i64).abs()).or_insert(0) += 1;
    }
    for (k, v) in &map {
      heap.push(*k);
    }

    let mut remain = k1 as i64 + k2 as i64;
    while remain > 0 {
      let v = heap.pop().unwrap();
      if v == 0 {
        break;
      }
      let mut cnt = *map.get(&v).unwrap();
      let diff = remain.min(cnt);
      cnt -= diff;
      remain -= diff;
      *map.get_mut(&v).unwrap() -= diff;
      if map.contains_key(&(v - 1)) {
        *map.get_mut(&(v - 1)).unwrap() += diff;
      } else {
        map.insert((v - 1), diff);
        heap.push(v - 1);
      }
    }
    let mut ans = 0i64;
    for (k, v) in map {
      ans += k * k * v;
    }

    ans
  }

  pub fn min_sum_square_diff2(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
    let mut m: HashMap<i64, i64> = HashMap::new();
    for i in 0..nums1.len() {
      m.entry((nums1[i] - nums2[i]).abs() as i64).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut buf: Vec<(i64, i64)> = vec![];
    let mut sum: i64 = 0;
    for (k, v) in m.iter() {
      buf.push((*k, *v));
      sum += k * k * v;
    }
    buf.sort_unstable();

    let mut t = (k1 + k2) as i64;
    let mut idx: usize = buf.len() - 1;
    while t > 0 && idx > 0 {
      let diff = buf[idx].0 - buf[idx - 1].0;
      let total = t.min(diff * buf[idx].1);

      let d = total / buf[idx].1;
      if total == diff * buf[idx].1 {
        sum -= diff * (buf[idx].0 + buf[idx - 1].0) * buf[idx].1;
        buf[idx - 1].1 += buf[idx].1;
        idx -= 1;
      } else {
        sum -= d * (buf[idx].0 * 2 - d) * buf[idx].1;
        sum -= ((buf[idx].0 - d) * 2 - 1) * (t - (d * buf[idx].1));
      }
      t -= total;
    }

    if t > 0 && buf[idx].0 > 0 {
      let total = t.min(buf[idx].0 * buf[idx].1);

      let d = total / buf[idx].1;
      if total == buf[idx].0 * buf[idx].1 {
        sum -= buf[idx].0 * buf[idx].0 * buf[idx].1;
      } else {
        sum -= d * (buf[idx].0 * 2 - d) * buf[idx].1;
        sum -= ((buf[idx].0 - d) * 2 - 1) * (t - (d * buf[idx].1));
      }
    }

    sum
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::min_sum_square_diff(vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1)
  );
}
