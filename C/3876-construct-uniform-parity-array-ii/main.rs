impl Solution {
  pub fn uniform_array(nums1: Vec<i32>) -> bool {
    let mut mn = nums1[0];
    let mut odd = 0u16;
    for &n in &nums1 {
      odd += if n % 2 == 0 { 0 } else { 1 };
      mn = mn.min(n);
    }

    mn & 1 == 1 || odd == 0
  }
}
