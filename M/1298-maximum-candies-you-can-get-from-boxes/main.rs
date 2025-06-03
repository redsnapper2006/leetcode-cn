use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
  pub fn max_candies(
    status: Vec<i32>,
    candies: Vec<i32>,
    keys: Vec<Vec<i32>>,
    contained_boxes: Vec<Vec<i32>>,
    initial_boxes: Vec<i32>,
  ) -> i32 {
    let mut pending: HashSet<usize> = HashSet::new();
    let mut visit: HashSet<usize> = HashSet::new();
    let mut status = status;
    let mut ans: i32 = 0;

    for ib in initial_boxes {
      let ib = ib as usize;
      if status[ib] == 0 {
        pending.insert(ib);
        continue;
      }

      if visit.contains(&ib) {
        continue;
      }

      let mut q: VecDeque<usize> = VecDeque::new();
      q.push_back(ib);
      while q.len() > 0 {
        let next = q.pop_front().unwrap();
        ans += candies[next];
        visit.insert(next);

        for cb in &contained_boxes[next] {
          let cb = *cb as usize;
          if visit.contains(&cb) {
            continue;
          }
          if status[cb] == 0 {
            pending.insert(cb);
            continue;
          }
          q.push_back(cb);
        }

        for key in &keys[next] {
          let key = *key as usize;
          if visit.contains(&key) {
            continue;
          }
          if status[key] == 0 {
            status[key] = 1;
            if pending.contains(&key) {
              q.push_back(key);
              pending.remove(&key);
            }
          }
        }
      }
    }

    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::max_candies(
      vec![1, 0, 1, 0],
      vec![7, 5, 4, 100],
      vec![vec![], vec![], vec![1], vec![]],
      vec![vec![1, 2], vec![3], vec![], vec![]],
      vec![0]
    )
  );

  println!(
    "{}",
    Solution::max_candies(
      vec![1, 0, 0, 0, 0, 0],
      vec![1, 1, 1, 1, 1, 1],
      vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
      vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
      vec![0]
    )
  );

  println!(
    "{}",
    Solution::max_candies(
      vec![1, 1, 1],
      vec![100, 1, 100],
      vec![vec![], vec![0, 2], vec![]],
      vec![vec![], vec![], vec![]],
      vec![1]
    )
  );

  println!(
    "{}",
    Solution::max_candies(vec![1], vec![100], vec![vec![]], vec![vec![]], vec![])
  );

  println!(
    "{}",
    Solution::max_candies(
      vec![1, 1, 1],
      vec![2, 3, 2],
      vec![vec![], vec![], vec![]],
      vec![vec![], vec![], vec![]],
      vec![2, 1, 0]
    )
  );
}
