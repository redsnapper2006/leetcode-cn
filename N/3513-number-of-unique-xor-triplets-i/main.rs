impl Solution {
  pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 2 {
      n as i32
    } else {
      n.next_power_of_two() as i32
    }
  }
}
