struct Solution {}

impl Solution {
  pub fn xor_game(nums: Vec<i32>) -> bool {
    let mut aggr: i32 = 0;

    for i in 0..nums.len() {
      aggr = aggr ^ nums[i];
    }

    aggr == 0 || nums.len() % 2 == 0
  }
}

fn main() {
  println!("{}", Solution::xor_game(vec![0; 5]));
}
