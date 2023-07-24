struct Solution {}

impl Solution {
  pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut buf : [i32; 58] = [0; 58];
    for c in jewels.chars() {
      buf[c as usize - 'A' as usize] = 1;
    }
    stones.chars().fold(0, |acc, c| acc + buf[c as usize - 'A' as usize])
  }
}
