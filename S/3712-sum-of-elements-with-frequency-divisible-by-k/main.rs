impl Solution {
  pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; 101];
    nums.iter().for_each(|&x| {
      buf[x as usize] += 1;
    });
    buf.iter().enumerate().fold(0, |sum, (idx, c)| {
      sum + if c % k == 0 { idx as i32 * c } else { 0 }
    })
  }
}
