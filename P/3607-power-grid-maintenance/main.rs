use std::collections::{HashMap, BTreeSet};

impl Solution {
  pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut egress: HashMap<i32, Vec<i32>> = HashMap::new();
    connections.iter().for_each(|conn| {
      egress.entry(conn[0]).and_modify(|x| x.push(conn[1])).or_insert(vec![conn[1]]);
      egress.entry(conn[1]).and_modify(|x| x.push(conn[0])).or_insert(vec![conn[0]]);
    });

    let c = c as usize;
    let mut index: Vec<i32> = vec![-1; c + 1];
    let mut vec_ts: Vec<BTreeSet<i32>> = vec![];
    let mut idx: i32 = 0;

    for i in 1..=c {
      if index[i] != -1 {
        continue;
      }
      let mut ts: BTreeSet<i32> = BTreeSet::new();
      build(i as i32, idx, &mut ts, &mut index, &egress);
      vec_ts.push(ts);
      idx += 1;
    }

    fn build(
      cur: i32, idx: i32, ts: &mut BTreeSet<i32>, index: &mut Vec<i32>,
      egress: &HashMap<i32, Vec<i32>>,
    ) {
      ts.insert(cur);
      index[cur as usize] = idx;
      if egress.contains_key(&cur) {
        egress.get(&cur).unwrap().iter().for_each(|&n| {
          if index[n as usize] != -1 {
            return;
          }

          build(n, idx, ts, index, egress);
        });
      }
    }

    let mut ans: Vec<i32> = vec![];
    queries.iter().for_each(|q| {
      let idx = index[q[1] as usize] as usize;
      if q[0] == 2 {
        vec_ts[idx].remove(&q[1]);
      } else if vec_ts[idx].contains(&q[1]) {
        ans.push(q[1]);
      } else {
        ans.push(if vec_ts[idx].len() == 0 {
          -1
        } else {
          *vec_ts[idx].first().unwrap()
        });
      }
    });

    ans
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::process_queries(
      5,
      vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
      vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]]
    )
  );

  println!(
    "{:?}",
    Solution::process_queries(3, vec![], vec![vec![1, 1], vec![2, 1], vec![1, 1]])
  );
}

struct Solution {}
