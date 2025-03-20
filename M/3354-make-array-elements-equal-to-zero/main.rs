impl Solution {
  pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
    let total = nums.iter().sum::<i32>();

    let mut sum: i32 = 0;
    let mut ans: i32 = 0;
    (0..nums.len()).for_each(|idx| {
      sum += nums[idx];
      if nums[idx] == 0 {
        if sum * 2 == total {
          ans += 2;
        } else if sum * 2 + 1 == total || sum * 2 - 1 == total {
          ans += 1;
        }
      }
    });

    ans
  }
}
