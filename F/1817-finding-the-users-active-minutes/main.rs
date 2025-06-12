use std::collections::HashMap;
impl Solution {
  pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut m: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    logs.iter().for_each(|log| {
      m.entry(log[0])
        .or_insert(HashMap::new())
        .entry(log[1])
        .and_modify(|x| (*x) += 1)
        .or_insert(1);
    });

    let mut ret: Vec<i32> = vec![0; k as usize];
    m.iter().for_each(|(_, v)| {
      ret[v.len() - 1] += 1;
    });
    ret
  }
}
