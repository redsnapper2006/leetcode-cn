struct Solution {}

impl Solution {
  pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    if (bloom_day.len() as i64) < (m as i64) * (k as i64) {
      return -1;
    }
    let mut start: i32 = 1;
    let mut end: i32 = *bloom_day.iter().max().unwrap();
    while start <= end {
      let mid = start + (end - start) / 2;
      let mut cnt: i32 = 0;
      let mut range: i32 = 0;
      (0..bloom_day.len()).for_each(|idx| {
        if bloom_day[idx] > mid {
          range = 0;
          return;
        }
        range += 1;
        if range >= k {
          cnt += 1;
          range = 0;
        }
      });

      if cnt >= m {
        end = mid - 1;
      } else {
        start = mid + 1;
      }
    }
    start
  }
}

fn main() {
  println!("{:?}", Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1));
  println!("{:?}", Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2));
  println!("{:?}", Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3));
  println!(
    "{:?}",
    Solution::min_days(vec![1000000000, 1000000000], 1, 1)
  );
  println!(
    "{:?}",
    Solution::min_days(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6], 4, 2)
  );
}
