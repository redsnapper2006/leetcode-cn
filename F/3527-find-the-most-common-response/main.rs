use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
  pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
    let coll = responses
      .iter()
      .map(|resp| HashSet::from_iter(resp.iter().cloned()))
      .collect::<Vec<HashSet<String>>>();

    let mut m: HashMap<String, i32> = HashMap::new();
    for col in coll {
      for k in col {
        m.entry(k).and_modify(|x| *x += 1).or_insert(1);
      }
    }
    let mut ans: String = "".to_string();
    let mut cnt: i32 = 0;
    for (k, v) in m {
      if v > cnt {
        cnt = v;
        ans = k;
      } else if v == cnt && ans > k {
        ans = k;
      }
    }
    ans
  }
}
