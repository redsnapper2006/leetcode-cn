struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut ans: i64 = 0;

    let mut cached1: HashMap<i32, i64> = HashMap::new();
    nums1.iter().for_each(|&v| {
      if v % k != 0 {
        return;
      }
      cached1.entry(v / k).and_modify(|x| *x += 1).or_insert(1);
    });

    let mut cached2: HashMap<i32, i64> = HashMap::new();
    nums2.iter().for_each(|&v| {
      cached2.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });
    // println!("{:?} {:?}", cached1, cached2);
    let mut dp: Vec<i64> = vec![0; 1000001];
    cached2.iter().for_each(|(&k, &v)| {
      let mut b = k;
      while b < 1000001 {
        dp[b as usize] += v;
        b += k;
      }
    });
    cached1.iter().for_each(|(&k, &v)| {
      ans += dp[k as usize] * v;
    });

    ans
  }
}

fn main() {
  println!(
    "{}",
    Solution::number_of_pairs(vec![24, 20, 24], vec![14, 8, 14], 3)
  );
  println!(
    "{}",
    Solution::number_of_pairs(vec![1, 22, 44, 5, 14], vec![6, 1, 22, 6, 23], 2)
  );
}
