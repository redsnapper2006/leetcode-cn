struct Solution {}

impl Solution {
  pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
    let mut max: i32 = 0;

    let mut cc: Vec<i32> = coins;
    cc.sort();

    for c in cc {
      if max + 1 < c {
        return max + 1;
      }
      max += c;
    }
    max + 1
  }
}
