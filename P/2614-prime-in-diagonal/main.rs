struct Solution {}

impl Solution {
  pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
    fn is_prime(n: i32) -> bool {
      n == 2 || n > 1 && n % 2 > 0 && (3..=(n as f64).sqrt() as i32).step_by(2).all(|i| n % i > 0)
    }

    *(0..nums.len()).map(|idx| {
      match (is_prime(nums[idx][idx]), is_prime(nums[idx][nums.len()-1-idx])) {
        (true, true) => nums[idx][idx].max(nums[idx][nums.len()-1-idx]),
        (false, true) => nums[idx][nums.len()-1-idx],
        (true, false) => nums[idx][idx],
        (false, false) => 0,
      }
    }).collect::<Vec<i32>>().iter().max().unwrap()
  }
}
