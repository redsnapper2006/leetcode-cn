impl Solution {
  pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
    let mut one_matrix: i128 = 0;
    let mut zero_matrix: i128 = 0;
    let mut minus_matrix: i128 = 0;

    pattern.iter().for_each(|&v| {
      one_matrix *= 2;
      zero_matrix *= 2;
      minus_matrix *= 2;

      if v == 1 {
        one_matrix += 1;
      } else if v == 0 {
        zero_matrix += 1;
      } else {
        minus_matrix += 1;
      }
    });
    let mut one: i128 = 0;
    let mut zero: i128 = 0;
    let mut minus: i128 = 0;
    let size: i32 = pattern.len() as i32;
    let base: i128 = (1 << size) - 1;

    let mut ans: i32 = 0;
    for i in 1..nums.len() {
      one *= 2;
      one &= base;
      zero *= 2;
      zero &= base;
      minus *= 2;
      minus &= base;
      if nums[i] > nums[i - 1] {
        one += 1;
      } else if nums[i] == nums[i - 1] {
        zero += 1;
      } else {
        minus += 1;
      }

      ans += if one & one_matrix == one_matrix
        && zero & zero_matrix == zero_matrix
        && minus & minus_matrix == minus_matrix
      {
        1
      } else {
        0
      };
    }
    ans
  }
}
