impl Solution {
  pub fn smallest_balanced_index(nums: Vec<i32>) -> i32 {
    let mut sum: i64 = 0;
    for i in 0..nums.len() - 1 {
      sum += nums[i] as i64;
    }

    let mut product: i64 = 1;
    for i in (1..nums.len()).rev() {
      if sum == product {
        return i as i32;
      }
      sum -= nums[i - 1] as i64;
      if product > sum / nums[i] as i64 {
        break;
      }
      product *= nums[i] as i64;
    }
    -1
  }

  pub fn smallest_balanced_index2(nums: Vec<i32>) -> i32 {
    let mut sum: Vec<i64> = vec![0; nums.len() + 1];
    let mut product: Vec<i64> = vec![1; nums.len() + 1];
    for i in 0..nums.len() {
      sum[i + 1] = nums[i] as i64 + sum[i];
      product[nums.len() - 1 - i] =
        if i64::MAX / nums[nums.len() - 1 - i] as i64 <= product[nums.len() - i] {
          -1
        } else {
          nums[nums.len() - 1 - i] as i64 * product[nums.len() - i]
        };
    }
    for i in 0..nums.len() {
      if sum[i] == product[i + 1] {
        return i as i32;
      }
    }
    -1
  }
}
