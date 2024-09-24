impl Solution {
  pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
    let mut p1 = red;
    let mut p2 = blue;
    let mut t1: i32 = 1;
    while t1 % 2 == 1 && p1 >= t1 || t1 % 2 == 0 && p2 >= t1 {
      if t1 % 2 == 1 {
        p1 -= t1;
      } else {
        p2 -= t1;
      }
      t1 += 1;
    }

    p1 = blue;
    p2 = red;
    let mut t2: i32 = 1;
    while t2 % 2 == 1 && p1 >= t2 || t2 % 2 == 0 && p2 >= t2 {
      if t2 % 2 == 1 {
        p1 -= t2;
      } else {
        p2 -= t2;
      }
      t2 += 1;
    }
    t1.max(t2) - 1
  }
}
