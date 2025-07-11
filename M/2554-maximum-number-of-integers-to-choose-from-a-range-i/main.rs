use std::collections::HashSet;

impl Solution {
  pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    let mut sum: i32 = 0;
    let ban: HashSet<i32> = HashSet::from_iter(banned.iter().cloned());

    let mut b: i32 = 1;
    let mut ans: i32 = 0;
    while b <= n {
      if sum + b > max_sum {
        break;
      }
      if !ban.contains(&b) {
        sum += b;
        ans +=1;
      }
      b += 1;
    }

    ans
  }
}

struct Solution {}
fn main() {
  println!("{}", Solution::max_count(vec![1, 3], 7, 10));
}
