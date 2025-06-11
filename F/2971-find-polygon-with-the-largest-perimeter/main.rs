impl Solution {
  pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut sum: i64 = nums[0] as i64 + nums[1] as i64;
    let mut ans: i64 = 0;
    (2..nums.len()).for_each(|idx| {
      let cur = nums[idx] as i64;
      if sum > cur {
        ans = sum + cur;
      }
      sum += cur;
    });
    ans
  }
}
