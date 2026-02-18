impl Solution {
  pub fn has_alternating_bits(n: i32) -> bool {
    ((n ^ n >> 1) + 1) & n == 0
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::has_alternating_bits(9));
}
