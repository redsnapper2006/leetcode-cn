struct Solution {}

impl Solution {
  pub fn next_beautiful_number(n: i32) -> i32 {
    let mut n = n + 1;

    loop {
      // println!("{}", n);
      let mut m = n;
      let mut digits = [0; 10];

      let mut is_match: bool = true;
      while m > 0 {
        let mode = m % 10;
        m /= 10;
        digits[mode as usize] += 1;
        if digits[mode as usize] > mode {
          break;
        }
      }
      (0..10).for_each(|i| {
        if digits[i] != 0 && digits[i] != i as i32 {
          is_match = false;
        }
      });

      // println!("{:?}", digits);
      // println!("is_match: {}", is_match);
      if is_match {
        break;
      }

      n += 1;
    }
    n
  }
}

fn main() {
  println!("{}", Solution::next_beautiful_number(1));
  println!("{}", Solution::next_beautiful_number(1000));
  println!("{}", Solution::next_beautiful_number(3000));
}
