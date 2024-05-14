struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();

    tasks.iter().for_each(|&v| {
      *m.entry(v).or_insert(0) += 1;
    });

    let mut ans: i32 = 0;
    m.iter().for_each(|(_, &c)| {
      match c {
        1 => {
          ans += 100001;
        }
        _ => {
          ans += (c + 2) / 3;
        }
      };
    });
    match ans >= 100001 {
      true => -1,
      _ => ans,
    }
  }
}
