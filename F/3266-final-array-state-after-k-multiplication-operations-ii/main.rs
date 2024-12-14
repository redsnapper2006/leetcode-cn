use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    if multiplier == 1 {
      return nums;
    }

    let multiplier = multiplier as i64;
    let nn = nums.len() as i64;
    let MOD = 1000000007;
    let mut h: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    let mut max: i64 = 0;
    nums.iter().enumerate().for_each(|(idx, &v)| {
      h.push(Reverse((v as i64, idx)));
      max = max.max(v as i64);
    });

    let mut kk = k as i64;
    while kk > 0 && h.peek().unwrap().0 .0 < max {
      let (v, idx) = h.pop().unwrap().0;
      h.push(Reverse((v * multiplier, idx)));
      kk -= 1;
    }

    let POW = |x: i64, n: i64| -> i64 {
      let mut res: i64 = 1;
      let mut n = n;
      let mut x = x;
      while n > 0 {
        if n % 2 > 0 {
          res = res * x % MOD;
        }
        x = x * x % MOD;
        n /= 2;
      }
      res
    };

    let mut ans: Vec<i32> = vec![0; nums.len()];
    (0..nn).for_each(|ii| {
      let (v, idx) = h.pop().unwrap().0;
      ans[idx] = ((v % MOD) * POW(multiplier, ((kk / nn) + if ii < (kk % nn) { 1 } else { 0 }))
        % MOD) as i32;
    });

    ans
  }
}
