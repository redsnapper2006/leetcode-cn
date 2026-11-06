use std::collections::HashMap;

impl Solution {
  pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
    let mut logs = logs;
    logs.sort_by_key(|l| l[1]);

    let mut qs = queries.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
    qs.sort_by_key(|t| t.1);

    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let (mut start, mut end) = (0usize, 0usize);

    let mut ans: Vec<i32> = vec![0; qs.len()];

    for &(idx_q, q) in &qs {
      while end < logs.len() && logs[end][1] <= q {
        *cnt.entry(logs[end][0]).or_insert(0) += 1;
        end += 1;
      }

      while start < end && logs[start][1] < q - x {
        if let std::collections::hash_map::Entry::Occupied(mut e) = cnt.entry(logs[start][0]) {
          *e.get_mut() -= 1;
          if *e.get() == 0 {
            e.remove();
          }
        }
        start += 1;
      }
      ans[idx_q] = n - cnt.len() as i32;
    }
    ans
  }
}
