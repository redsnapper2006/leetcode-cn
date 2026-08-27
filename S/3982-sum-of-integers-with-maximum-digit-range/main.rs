impl Solution {
  pub fn max_digit_range(nums: Vec<i32>) -> i32 {
    let mut buf: [i32; 10] = [0i32; 10];
    for &nn in &nums {
      let mut n = nn;
      let mut mx = -1i32;
      let mut mn = 10i32;
      while n > 0 {
        let d = n % 10;
        mx = mx.max(d);
        mn = mn.min(d);
        n /= 10;
      }

      buf[(mx - mn) as usize] += nn;
    }
    for i in (0..10).rev() {
      if buf[i] != 0 {
        return buf[i];
      }
    }
    0
  }
}
