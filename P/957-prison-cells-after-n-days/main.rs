use std::collections::HashSet;

impl Solution {
  pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut digits: i32 = cells.iter().fold(0, |d, v| d * 2 + v);

    let mut m: HashSet<i32> = HashSet::new();
    let mut buf: Vec<i32> = vec![];
    let mut is_loop: bool = false;
    for _ in 0..n {
      digits = (((digits ^ (digits >> 2)) & 0x3f) ^ 0x3f) << 1;
      if m.contains(&digits) {
        is_loop = true;
        break;
      }
      m.insert(digits);
      buf.push(digits);
    }

    digits = if is_loop {
      buf[(n - 1 + buf.len()) % buf.len()]
    } else {
      digits
    };

    (0..8).rev().fold(vec![], |mut ans, i| {
      ans.push((digits >> i) & 1);
      ans
    })
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::prison_after_n_days(vec![1, 1, 0, 1, 1, 0, 0, 1], 300663720)
  );
}
