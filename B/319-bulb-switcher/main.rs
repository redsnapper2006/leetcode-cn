struct Solution {}

impl Solution {
  pub fn bulb_switch(n: i32) -> i32 {
    if n <= 1 {
      return n;
    }

    let mut count: i32 = 3;
    let mut now: i32 = 1;
    let mut current: i32 = 0;
    for _i in 2..n + 1 {
      if now < count {
        now += 1;
      } else {
        count += 2;
        current += 1;
        now = 1;
      }
    }

    current + 1
  }

  pub fn bulb_switch2(n: i32) -> i32 {
    (n as f64).sqrt() as i32
  }
}

fn main() {
  println!("{}", Solution::bulb_switch(4));
  println!("{}", Solution::bulb_switch(6));
  println!("{}", Solution::bulb_switch(100));
  println!("{}", Solution::bulb_switch2(1000000000));
}
