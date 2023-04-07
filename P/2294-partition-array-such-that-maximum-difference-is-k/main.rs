struct Solution {}

impl Solution {
  pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut m = nums;
    m.sort();

    let mut base: i32 = -1;
    let mut ret: i32 = 0;
    for v in nums {
      if v > base {
        base = v + k;
        ret += 1;
      }
    }

    ret
  }
}
