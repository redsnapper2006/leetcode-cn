struct Solution {}

impl Solution {
  pub fn judge_square_sum(c: i32) -> bool {
    let mut a: i32 = 0;
    let mut b: i32 = (c as f64).sqrt() as i32;
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
