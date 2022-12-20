struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
    let mut task: HashMap<i32, i32> = HashMap::new();

    let mut base: i32 = 0;
    for log in logs {
      let id = log[0];
      let duration = log[1] - base;

      let mut v = task.entry(duration).or_insert(id);
      if *v > id {
        *v = id;
      }
      base = log[1];
    }
    let max = task.keys().max().unwrap();
    *task.get(&max).unwrap()
  }
}
