impl Solution {
  pub fn mirror_reflection(p: i32, q: i32) -> i32 {
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
    let mut p = p;
    let mut q = q;
    let g = gcd(p, q);
    p /= g;
    q /= g;

    if p % 2 == 0 {
      return 2;
    }

    if q % 2 == 0 {
      return 0;
    }
    1
  }
}
