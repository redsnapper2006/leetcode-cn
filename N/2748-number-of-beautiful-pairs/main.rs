struct Solution {}

impl Solution {
  pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
    let mut f: Vec<i32> = Vec::new();
    let mut r: Vec<i32> = Vec::new();

    nums.iter().for_each(|&v| {
      r.push(v % 10);
      let mut v = v;
      while v > 9 {
        v /= 10;
      }
      f.push(v);
    });

    fn gcd(x: i32, y: i32) -> i32 {
      let mut n1 = x;
      let mut n2 = y;
      if (x < y) {
        n1 = y;
        n2 = x;
      }

      let mut rem = n1 % n2;
      while (rem != 0) {
        n1 = n2;
        n2 = rem;
        rem = n1 % n2;
      }
      n2
    }

    let mut ans: i32 = 0;

    (0..f.len()).for_each(|i| {
      (i + 1..f.len()).for_each(|j| {
        if gcd(f[i], r[j]) == 1 {
          ans += 1;
        }
      });
    });
    ans
  }
}
