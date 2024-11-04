struct Solution {}

impl Solution {
  pub fn judge_square_sum(c: i32) -> bool {
    let c = c as i64;
    let mut a: i64 = 0;
    let mut b: i64 = (c as f64).sqrt() as i64;
    while a <= b {
      let d = a * a + b * b;
      if d == c {
        return true;
      } else if d > c {
        b -= 1;
      } else {
        a += 1;
      }
    }
    false
  }
}

fn main() {
  println!("{}", Solution::judge_square_sum(5));
}
