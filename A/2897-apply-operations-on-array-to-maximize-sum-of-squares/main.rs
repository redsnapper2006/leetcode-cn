impl Solution {
  pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut bits: [i32; 30] = [0; 30];
    nums.iter().for_each(|&n| {
      let mut n = n;
      while n != 0 {
        bits[n.trailing_zeros() as usize] += 1;
        n &= n - 1;
      }
    });

    let mut ans = 0i64;
    for _ in 0..k {
      let mut s = 0i64;
      for offset in (0..30).rev() {
        if bits[offset] > 0 {
          s += 1 << offset;
          bits[offset] -= 1;
        }
      }
      ans = (ans + s * s) % 1000000007;
    }
    ans as i32
  }
}
