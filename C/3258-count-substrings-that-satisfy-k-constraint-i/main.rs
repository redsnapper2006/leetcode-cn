struct Solution {}

impl Solution {
  pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut left: usize = 0;
    let mut right: usize = 0;

    let mut zero: i32 = 0;
    let mut one: i32 = 0;
    let mut ans: i32 = 0;
    while right < bb.len() {
      if bb[right] == b'0' {
        zero += 1;
      } else {
        one += 1;
      }
      right += 1;

      while zero > k && one > k {
        if bb[left] == b'0' {
          zero -= 1;
        } else {
          one -= 1;
        }
        left += 1;
      }

      ans += (right - left) as i32;
    }

    ans
  }
}
