struct Solution {}

impl Solution {
  pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    nums
      .iter()
      .map(|&v| {
        let mut vv = v;
        let mut buf: Vec<i32> = Vec::new();
        while vv > 0 {
          buf.push(vv % 10);
          vv /= 10;
        }
        buf.reverse();
        buf
      })
      .collect::<Vec<Vec<i32>>>()
      .iter()
      .fold(Vec::new(), |mut acc, v| {
        acc.extend(v);
        acc
      })
  }
}
