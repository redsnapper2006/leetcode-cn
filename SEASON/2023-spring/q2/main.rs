struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn adventure_camp(expeditions: Vec<String>) -> i32 {
    let mut visited: HashSet<String> = HashSet::new();

    let first: Vec<&str> = expeditions[0].split("->").collect();
    first.iter().for_each(|v| {
      visited.insert(v.to_string());
    });

    let mut res: i32 = -1;
    let mut min_idx: i32 = -1;
    (1..expeditions.len()).for_each(|idx| {
      let mut time_res: i32 = 0;
      let camp: Vec<&str> = expeditions[idx].split("->").collect();
      camp.iter().for_each(|v| {
        let vv = v.to_string();
        if vv.is_empty() {
          return;
        }

        if !visited.contains(&vv) {
          time_res += 1;
        }
        visited.insert(vv);
      });

      if time_res > 0 && time_res > res {
        res = time_res;
        min_idx = idx as i32;
      }
    });

    min_idx
  }
}
