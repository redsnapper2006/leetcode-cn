impl Solution {
  pub fn closest_divisors(num: i32) -> Vec<i32> {
    let n1 = num + 1;
    let mut m1 = (n1 as f64).sqrt() as i32;

    let mut ans: i32 = i32::MAX;
    let mut ret: Vec<i32> = vec![];
    while m1 >= 1 {
      if n1 % m1 == 0 {
        ans = ((n1 / m1) - m1).abs();
        ret = vec![(n1 / m1), m1];
        break;
      }
      m1 -= 1;
    }

    let n2 = num + 2;
    let mut m2 = (n2 as f64).sqrt() as i32;
    while m2 >= 1 {
      if n2 % m2 == 0 {
        if ans > ((n2 / m2) - m2).abs() {
          ret = vec![(n2 / m2), m2];
        }
        break;
      }
      m2 -= 1;
    }
    ret
  }
}
