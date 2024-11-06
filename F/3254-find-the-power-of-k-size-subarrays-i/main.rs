struct Solution {}

impl Solution {
  pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    nums
      .iter()
      .enumerate()
      .fold((0, Vec::<i32>::new()), |(cnt, mut ans), (idx, v)| {
        if idx + 1 >= k as usize {
          if k == 1 {
            ans.push(nums[idx]);
          } else if nums[idx] == nums[idx - 1] + 1 {
            ans.push(if cnt + 1 >= k { nums[idx] } else { -1 });
          } else {
            ans.push(-1);
          }
        }
        if k > 1 && idx > 0 && nums[idx] == nums[idx - 1] + 1 {
          (cnt + 1, ans)
        } else {
          (1, ans)
        }
      })
      .1
  }
}
