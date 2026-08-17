impl Solution {
  pub fn kth_digit(k: i64) -> i32 {
    let mut count: i64 = 9;
    let mut start: i64 = 1;
    let mut digits: i64 = 1;
    let mut k = k;
    while k > count * digits {
      k -= count * digits;
      start *= 10;
      digits += 1;
      count *= 10;
    }

    let num = start + (k - 1) / digits;
    let pos = (k - 1) % digits;
    let digit = (num / 10i64.pow((digits - 1 - pos) as u32)) % 10;

    if pos == digits - 1 && (num / 10) % 2 == 1 {
      (9 - digit) as i32
    } else {
      digit as i32
    }
  }

  pub fn kth_digit2(k: i64) -> i32 {
    if k < 10 {
      return k as i32;
    }
    let mut count: i64 = 9;
    let mut start: i64 = 1;
    let mut digits: i64 = 1;
    let mut k = k;
    while k > count * digits {
      k -= count * digits;
      start *= 10;
      digits += 1;
      count *= 10;
    }

    let mut evenodd = start / 10 + (k - 1) / (10 * digits);

    if k % digits == 0 {
      k /= digits;
      k -= 1;
      k %= 10;
      if evenodd % 2 == 0 {
        k as i32
      } else {
        (9 - k) as i32
      }
    } else {
      k %= digits;

      k = digits - 1 - k;
      while k > 0 {
        evenodd /= 10;
        k -= 1;
      }
      (evenodd % 10) as i32
    }
  }
}

struct Solution {}

fn main() {
  println!("{} : 4", Solution::kth_digit(4));
  println!("{} : 7", Solution::kth_digit(15));
  println!("{} : 9", Solution::kth_digit(11));
  println!("{} : 2", Solution::kth_digit(25));
  println!("{} : 2", Solution::kth_digit(36));
  println!("{} : 4", Solution::kth_digit(78));
  println!("{} : 1", Solution::kth_digit(855));
  println!("{} : 0", Solution::kth_digit(29));
  println!("{} : 9", Solution::kth_digit(49));
  println!("{} : 8", Solution::kth_digit(168));
  println!("{} : 8", Solution::kth_digit(1088888888778));
}
