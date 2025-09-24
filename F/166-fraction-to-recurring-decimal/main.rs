use std::collections::HashMap;

impl Solution {
  pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    let mut idx_map: HashMap<i64, usize> = HashMap::new();

    let mut ans: Vec<u8> = vec![];
    if numerator > 0 && denominator < 0 || numerator < 0 && denominator > 0 {
      ans.push(b'-');
    }
    let numerator = numerator as i64;
    let denominator = denominator as i64;
    let numerator = if numerator < 0 { -numerator } else { numerator };
    let denominator = if denominator < 0 {
      -denominator
    } else {
      denominator
    };

    let div = numerator / denominator;
    for &b in format!("{}", div).as_bytes().iter() {
      ans.push(b);
    }
    if div * denominator == numerator {
      return String::from_utf8(ans).unwrap();
    }

    ans.push(b'.');
    let mut num = numerator;
    num %= denominator;
    idx_map.insert(num, ans.len());
    num *= 10;
    while num > 0 {
      ans.push(b'0' + (num / denominator) as u8);
      num %= denominator;
      if idx_map.contains_key(&num) {
        ans.insert(*idx_map.get(&num).unwrap(), b'(');
        ans.push(b')');
        break;
      } else {
        idx_map.insert(num, ans.len());
        num *= 10;
      }
    }

    return String::from_utf8(ans).unwrap();
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::fraction_to_decimal(1, 3));
  println!("{}", Solution::fraction_to_decimal(1, 2));
  println!("{}", Solution::fraction_to_decimal(4, 2));
  println!("{}", Solution::fraction_to_decimal(4, 333));
  println!("{}", Solution::fraction_to_decimal(-50, 8));
  println!("{}", Solution::fraction_to_decimal(7, -12));

  println!("{}", Solution::fraction_to_decimal(-1, -2147483648));
}
