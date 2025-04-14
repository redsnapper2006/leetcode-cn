impl Solution {
  pub fn count_partitions(nums: Vec<i32>) -> i32 {
    let total = nums.iter().sum::<i32>();
    let mut ans: i32 = 0;
    let mut sum: i32 = 0;
    (0..nums.len() - 1).for_each(|idx| {
      sum += nums[idx];
      ans += if ((total - sum * 2) % 2 == 0) { 1 } else { 0 };
    });
    ans
  }

  pub fn count_partitions(nums: Vec<i32>) -> i32 {
    if nums.iter().sum::<i32>() % 2 == 0 {
      nums.len() as i32 - 1
    } else {
      0
    }
  }
}
