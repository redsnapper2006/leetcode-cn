struct Solution {}

impl Solution {
  pub fn minimum_swap(s1: String, s2: String) -> i32 {
    let (x, y): (i32, i32) = s1.chars().zip(s2.chars()).filter(|(b1, b2)| b1 != b2).fold(
      (0, 0),
      |(x, y), (b1, b2)| match (b1, b2) {
        ('x', _) => (x + 1, y),
        ('y', _) => (x, y + 1),
        (_, _) => (x, y),
      },
    );

    // println!("{} {}", x, y);
    if x % 2 != y % 2 {
      return -1;
    }
    x / 2 + y / 2 + (x % 2) * 2
  }
}

fn main() {
  for (x, y) in vec![
    ("xy".to_string(), "yx".to_string()),
    ("xx".to_string(), "yy".to_string()),
    ("xx".to_string(), "xy".to_string()),
    ("xxyxyxyxx".to_string(), "xyxyxxxyx".to_string()),
  ] {
    println!("{}", Solution::minimum_swap(x, y));
  }
}
