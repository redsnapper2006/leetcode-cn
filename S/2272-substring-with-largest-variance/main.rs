struct Solution {}

impl Solution {
  pub fn largest_variance(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();

    let mut ans: i32 = 0;
    (0..26).for_each(|i| {
      let a = (b'a' + i) as u8;
      (0..26).for_each(|j| {
        let b = (b'a' + j) as u8;
        if a == b {
          return;
        }

        let mut f0: i32 = 0;
        let mut f1: i32 = -100000000;

        bb.iter().for_each(|&v| {
          if v == a {
            f0 = f0.max(0) + 1;
            f1 += 1;
          }
          if v == b {
            let v = f0.max(0) - 1;
            f0 = v;
            f1 = v;
          }
          ans = ans.max(f1);
        });
      });
    });
    ans
  }
}
