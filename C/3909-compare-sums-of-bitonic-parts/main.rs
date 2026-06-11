impl Solution {
  pub fn compare_bitonic_sums(nums: Vec<i32>) -> i32 {
    let mut asc: i64 = 0;
    let mut desc: i64 = 0;
    let mut base: i64 = 0;
    nums.iter().for_each(|&v| {
      let v = v as i64;
      if v > base {
        asc += v;
        desc = v;
      } else {
        desc += v;
      }
      base = v;
    });
    if asc > desc {
      0
    } else if asc < desc {
      1
    } else {
      -1
    }
  }
}
