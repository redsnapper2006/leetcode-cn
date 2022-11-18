struct Solution {}

impl Solution {
  pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
    let _mod: i128 = 1000000007;
    let mut sum: i128 = 0;
    let mut nn = nums.clone();
    let n = nn.len();
    nn.sort();
    let mut pow2: Vec<i128> = vec![1; nums.len()];
    for i in 1..nn.len() {
      pow2[i] = pow2[i - 1] * 2 % _mod;
    }

    for (i, v) in nn.iter().enumerate() {
      sum += (pow2[i] - pow2[n - 1 - i]) * *v as i128;
    }

    ((sum % _mod + _mod) % _mod) as i32
  }
}

fn main() {
  println!(
    "{}",
    Solution::sum_subseq_widths(vec![
      5, 69, 89, 92, 31, 16, 25, 45, 63, 40, 16, 56, 24, 40, 75, 82, 40, 12, 50, 62, 92, 44, 67,
      38, 92, 22, 91, 24, 26, 21, 100, 42, 23, 56, 64, 43, 95
    ])
  );
}
