use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
  pub fn find_all_people(_n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    let mut meetings = meetings;
    meetings.sort_by(|x, y| x[2].cmp(&y[2]));

    let mut conn: HashMap<i32, HashMap<i32, Vec<i32>>> = HashMap::new();
    meetings.iter().for_each(|mt| {
      conn
        .entry(mt[0])
        .or_insert(HashMap::new())
        .entry(mt[1])
        .or_insert(vec![])
        .push(mt[2]);
      conn
        .entry(mt[1])
        .or_insert(HashMap::new())
        .entry(mt[0])
        .or_insert(vec![])
        .push(mt[2]);
    });

    let mut ans: HashMap<i32, i32> = HashMap::new();
    ans.insert(0, 0);
    ans.insert(first_person, 0);
    let mut q: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    q.push(Reverse((0, 0)));
    q.push(Reverse((0, first_person)));

    while q.len() > 0 {
      let cur = q.pop().unwrap().0;
      if !conn.contains_key(&cur.1) {
        continue;
      }

      for (&nxt_k, nxt_v) in conn.get(&cur.1).unwrap() {
        if nxt_v[nxt_v.len() - 1] < cur.0
          || ans.contains_key(&nxt_k)
            && (*ans.get(&nxt_k).unwrap() <= cur.0
              || *ans.get(&nxt_k).unwrap() <= nxt_v[nxt_v.len() - 1])
        {
          continue;
        }
        let ll = match nxt_v.binary_search(&cur.0) {
          Ok(ov) => ov,
          Err(ev) => ev,
        };
        ans.insert(nxt_k, nxt_v[ll]);
        q.push(Reverse((nxt_v[ll], nxt_k)));
      }
    }

    let mut ans = ans
      .into_iter()
      .collect::<Vec<(i32, i32)>>()
      .iter()
      .map(|x| x.0)
      .collect::<Vec<_>>();
    ans.sort_unstable();
    ans
  }
}

struct Solution {}
fn main() {
  println!(
    "{:?}",
    Solution::find_all_people(6, vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]], 1)
  );
  println!(
    "{:?}",
    Solution::find_all_people(6, vec![vec![3, 1, 3], vec![1, 2, 2], vec![0, 3, 3]], 3)
  );
  println!(
    "{:?}",
    Solution::find_all_people(6, vec![vec![3, 4, 2], vec![1, 2, 1], vec![2, 3, 1]], 1)
  );
}
