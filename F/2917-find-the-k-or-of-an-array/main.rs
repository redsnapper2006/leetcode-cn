
impl Solution {
  pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; 32];

    nums.iter().for_each(|num| {
      (0..32).for_each(|offset| {
        if num & (1 << offset) > 0 {
          buf[offset] += 1;
        }
      });
    });

    let mut v: i32 = 0;
    (0..32).for_each(|offset| {
      if buf[offset] >= k {
        v += (1 << offset);
      }
    });
    v
  }
}
