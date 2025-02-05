struct Solution {}

impl Solution {
  pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ans: Vec<Vec<i32>> = vec![vec![]];
    let mut prev: usize = 0;
    (0..nums.len()).for_each(|idx| {
      let (start, end) = if idx > 0 && nums[idx] == nums[idx - 1] {
        (prev, ans.len())
      } else {
        (0, ans.len())
      };
      prev = ans.len();
      (start..end).for_each(|ii| {
        let mut candi = ans[ii].clone();
        candi.push(nums[idx]);
        ans.push(candi);
      });
    });
    ans
  }
}
