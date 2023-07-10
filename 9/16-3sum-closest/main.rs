struct Solution {}

impl Solution {
  pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nn = nums;
    nn.sort();

    let mut min: i32 = 1 << 31 - 1;
    let mut res : i32 = 0;
    let mut idx: usize = 0;
    while idx < nn.len() {
      let mut l = idx + 1;
      let mut r = nn.len() - 1;
      while l < r {
        let sum = nn[idx] + nn[l] + nn[r];
        let diff = (sum - target).abs();

        if diff < min {
          min = diff;
          res = sum;
        }
        if sum == target {
          return sum;
        } else if sum < target {
          l += 1;
        } else {
          r -= 1;
        }
      }
      idx += 1;
    }
    res
  }
}
