impl Solution {
  pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() <= 2 {
      return vec![];
    }
    let mut nums = nums;
    nums.sort_unstable();

    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 0..nums.len() - 2 {
      if i > 0 && nums[i] == nums[i - 1] {
        continue;
      }

      let mut j: usize = i + 1;
      let mut k: usize = nums.len() - 1;
      while j < k {
        if j > i + 1 && nums[j] == nums[j - 1] {
          j += 1;
          continue;
        }
        if k < nums.len() - 1 && nums[k] == nums[k + 1] {
          k -= 1;
          continue;
        }

        let sum = nums[i] + nums[j] + nums[k];
        if sum == 0 {
          ans.push(vec![nums[i], nums[j], nums[k]]);
          j += 1;
          if k == 0 {
            break;
          }
          k -= 1;
        } else if sum > 0 {
          if k == 0 {
            break;
          }
          k -= 1;
        } else {
          j += 1;
        }
      }
    }
    ans
  }
}
