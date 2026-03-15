impl Solution {
  pub fn smallest_absent(nums: Vec<i32>) -> i32 {
    let mut buf: Vec<i32> = vec![0; 101];
    let mut sum: i32 = 0;
    nums.iter().for_each(|&v| {
      if v >= 0 {
        buf[v as usize] = 1;
      }
      sum += v;
    });
    let mut base: i32 = (sum) / nums.len() as i32 + 1;
    if base < 1 {
      base = 1;
    }
    while base <= 100 {
      if buf[base as usize] == 0 {
        break;
      }
      base += 1;
    }
    base
  }
}
