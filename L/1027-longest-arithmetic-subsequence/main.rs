struct Solution {}

impl Solution {
  pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<[i32; 1001]> = vec![[0; 1001]; nums.len()];
    let mut max: i32 = 0;
    (1..nums.len()).for_each(|idx| {
      (0..idx).for_each(|pidx| {
        let offset = (nums[idx] - nums[pidx] + 500) as usize;
        dp[idx][offset] = dp[pidx][offset] + 1;
        if dp[idx][offset] > max {
          max = dp[idx][offset];
        }
      });
    });
    // println!("{:?}", dp);
    max + 1
  }
}

fn main() {
  println!("{}", Solution::longest_arith_seq_length(vec![3, 6, 9, 12]));

  println!(
    "{}",
    Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10])
  );

  println!(
    "{}",
    Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8])
  );
}
