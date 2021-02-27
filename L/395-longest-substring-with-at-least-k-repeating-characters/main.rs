struct Solution {}

impl Solution {
  pub fn longest_substring(s: String, k: i32) -> i32 {
    let mut ret = 0;
    let bb = s.as_bytes();
    for t in 1..27 {
      let mut l: usize = 0;
      let mut r: usize = 0;
      let mut tot = 0;
      let mut less = 0;
      let mut buf: Vec<i32> = vec![0; 26];

      while r < bb.len() {
        buf[(bb[r] - 'a' as u8) as usize] += 1;
        if buf[(bb[r] - 'a' as u8) as usize] == 1 {
          tot += 1;
          less += 1;
        }
        if buf[(bb[r] - 'a' as u8) as usize] == k {
          less -= 1;
        }
        while tot > t {
          buf[(bb[l] - 'a' as u8) as usize] -= 1;
          if buf[(bb[l] - 'a' as u8) as usize] == k - 1 {
            less += 1;
          }
          if buf[(bb[l] - 'a' as u8) as usize] == 0 {
            tot -= 1;
            less -= 1;
          }
          l += 1;
        }

        if less == 0 {
          if ret < r - l + 1 {
            ret = r - l + 1;
          }
        }
        r += 1;
      }
    }
    ret as i32
  }
}

fn main() {
  println!(
    "{}",
    Solution::longest_substring(String::from("helloworld"), 3)
  );
}
