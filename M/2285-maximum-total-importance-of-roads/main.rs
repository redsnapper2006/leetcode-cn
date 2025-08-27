use std::collections::HashMap;
impl Solution {
  pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let mut n = n as i64;
    let mut m: HashMap<i32, i64> = HashMap::new();
    roads.iter().for_each(|road| {
      m.entry(road[0]).and_modify(|x| *x += 1).or_insert(1);
      m.entry(road[1]).and_modify(|x| *x += 1).or_insert(1);
    });

    let mut buf: Vec<i64> = vec![];
    for (_, v) in m.iter() {
      buf.push(*v as i64);
    }
    buf.sort_unstable();
    let mut ans: i64 = 0;
    for i in (0..buf.len()).rev() {
      ans += buf[i] * n;
      n -= 1;
    }
    ans
  }
}
