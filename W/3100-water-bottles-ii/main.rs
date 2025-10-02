impl Solution {
  pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
    let mut ans: i32 = 0;
    while num_bottles >= num_exchange {
      ans += num_exchange;
      num_bottles -= num_exchange - 1;
      num_exchange += 1;
    }
    ans + num_bottles
  }
}
