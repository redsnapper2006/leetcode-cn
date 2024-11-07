struct Solution {}

impl Solution {
  pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    nums
      .iter()
      .enumerate()
      .fold((0, Vec::<i32>::new()), |(mut cnt, mut ans), (idx, v)| {
        if idx == 0 || nums[idx] != nums[idx - 1] + 1 {
          cnt = 1;
        } else {
          cnt += 1;
        }
        if cnt >= k as usize {
          ans.push(nums[idx]);
        } else if idx + 1 >= k as usize {
          ans.push(-1);
        }
        (cnt, ans)
      })
      .1
  }
}
