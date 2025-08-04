impl Solution {
  pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut b1: (i32, i32) = (-1, 0);
    let mut b2: (i32, i32) = (-1, 0);
    let mut ans: i32 = 0;
    let mut s: usize = 0;
    let mut e: usize = 0;
    while e < fruits.len() {
      let f = fruits[e];
      if b1.0 == -1 || b1.0 == f {
        b1.0 = f;
        b1.1 += 1;
      } else if b2.0 == -1 || b2.0 == f {
        b2.0 = f;
        b2.1 += 1;
      } else {
        while s < e && b1.1 > 0 && b2.1 > 0 {
          if fruits[s] == b1.0 {
            b1.1 -= 1;
          } else {
            b2.1 -= 1;
          }
          s += 1;
        }
        if b1.1 == 0 {
          b1.0 = f;
          b1.1 = 1;
        } else {
          b2.0 = f;
          b2.1 = 1;
        }
      }

      ans = ans.max(e - s + 1);
      e += 1;
    }
    ans
  }
}
