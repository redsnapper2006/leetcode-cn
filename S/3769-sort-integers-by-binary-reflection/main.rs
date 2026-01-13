impl Solution {
  pub fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
    let mut buf = nums
      .iter()
      .map(|&n| {
        let mut m = n;
        let mut t: i32 = 0;
        while m > 0 {
          t <<= 1;
          t += m & 1;
          m >>= 1;
        }
        (t, n)
      })
      .collect::<Vec<(i32, i32)>>();

    buf.sort_by(|&p1, &p2| {
      if p1.0 == p2.0 {
        p1.1.cmp(&p2.1)
      } else {
        p1.0.cmp(&p2.0)
      }
    });
    buf.iter().map(|x| x.1).collect::<Vec<i32>>()
  }
}
