struct Solution {}

impl Solution {
  pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
    let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect::<Vec<i64>>();
    let x: i64 = x as i64;
    let mut even = nums[0] - x;
    let mut odd = nums[0] - x;
    match nums[0] % 2 {
      0 => even = nums[0],
      _ => odd = nums[0],
    };

    (1..nums.len()).for_each(|idx| {
      if nums[idx] % 2 == nums[idx - 1] % 2 {
        match nums[idx] % 2 {
          0 => even += nums[idx],
          _ => odd += nums[idx],
        };
      } else {
        match nums[idx] % 2 {
          0 => {
            even += nums[idx];
            even = even.max(odd + nums[idx] - x);
          }
          _ => {
            odd += nums[idx];
            odd = odd.max(even + nums[idx] - x);
          }
        };
      }
    });
    even.max(odd)
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_score(
      vec![8, 50, 65, 85, 8, 73, 55, 50, 29, 95, 5, 68, 52, 79],
      74
    )
  );
}
