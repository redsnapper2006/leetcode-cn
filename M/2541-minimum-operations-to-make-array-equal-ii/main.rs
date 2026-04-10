impl Solution {
  pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut plus: i64 = 0;
    let mut minus: i64 = 0;
    for i in 0..nums1.len() {
      let d = nums1[i] - nums2[i];
      if k > 0 && d % k != 0 || k == 0 && d != 0 {
        return -1;
      }
      if d >= 0 {
        plus += d as i64;
      } else {
        minus += (-d) as i64;
      }
    }
    if plus != minus || plus > 0 && k == 0 {
      -1
    } else {
      if plus == 0 { 0 } else { plus / (k as i64) }
    }
  }
}
