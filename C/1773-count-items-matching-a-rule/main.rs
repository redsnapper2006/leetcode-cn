struct Solution {}

impl Solution {
  pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let mut ret: i32 = 0;
    let mut idx = 0;
    if rule_key == "type" {
      idx = 0;
    } else if rule_key == "color" {
      idx = 1;
    } else {
      idx = 2;
    }
    for i in 0..items.len() {
      if items[i][idx] == rule_value {
        ret += 1;
      }
    }
    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::count_matches(
      vec![vec![String::from(""); 5]; 5],
      String::from("type"),
      String::from("phone")
    )
  );
}
