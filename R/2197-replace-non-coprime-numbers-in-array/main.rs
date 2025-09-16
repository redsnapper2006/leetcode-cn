impl Solution {
  pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![nums[0]];

    fn gcd(x: i64, y: i64) -> i64 {
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

    for idx in 1..nums.len() {
      ans.push(nums[idx]);

      while ans.len() > 1 {
        let d1 = ans[ans.len() - 1] as i64;
        let d2 = ans[ans.len() - 2] as i64;

        let g = gcd(d1, d2);
        if g > 1 {
          ans.pop();
          ans.pop();
          ans.push((d1 * d2 / g) as i32);
        } else {
          break;
        }
      }
    }
    ans
  }
}
