impl Solution {
  pub fn count_ratio_subarrays(nums: Vec<i32>, a: i32, b: i32) -> i32 {
    let mut buf: Vec<(i32, i32)> = vec![];
    let (mut even, mut odd): (i32, i32) = (0, 0);
    for &n in &nums {
      even += if n & 1 == 0 { 1 } else { 0 };
      odd += if n & 1 == 1 { 1 } else { 0 };
      buf.push((even, odd));
    }
    let mut ans: i32 = 0;
    for i in 0..nums.len() {
      let base = if i > 0 { buf[i - 1] } else { (0, 0) };

      for j in i..nums.len() {
        let (diffe, diffo) = (buf[j].0 - base.0, buf[j].1 - base.1);
        ans += if diffo > 0 && diffo * a >= diffe * b {
          1
        } else {
          0
        };
      }
    }
    ans
  }
}
