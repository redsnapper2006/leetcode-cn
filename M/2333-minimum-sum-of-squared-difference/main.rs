use std::collections::HashMap;
impl Solution {
  pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
    let mut m: HashMap<i64, i64> = HashMap::new();
    for i in 0..nums1.len() {
      m.entry((nums1[i] - nums2[i]).abs() as i64)
        .and_modify(|x| *x += 1)
        .or_insert(1);
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
