struct Solution {}

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
  #[allow(unused_variables)]
  pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
    let mut connect: HashMap<i32, Vec<i32>> = HashMap::new();

    edges.iter().for_each(|x| {
      connect.entry(x[0]).or_insert(Vec::new()).push(x[1]);
      connect.entry(x[1]).or_insert(Vec::new()).push(x[0]);
    });

    let mut queue: VecDeque<(i32, HashSet<i32>, i32, f64)> = VecDeque::new();
    let mut s: HashSet<i32> = HashSet::new();
    s.insert(1);
    queue.push_back((1, s, 0, 1.0));

    let mut res: f64 = 0.0;
    while queue.len() > 0 {
      let (candi, visit, step, prop) = queue.pop_front().unwrap();
      if candi == target && step == t {
        res += prop;
      }

      let next = match connect.get(&candi) {
        None => Vec::new(),
        Some(next) => next.clone(),
      };

      let mut nextcnt: i32 = 0;
      next.iter().for_each(|&x| {
        if !visit.contains(&x) {
          nextcnt += 1;
        }
      });

      if step + 1 > t {
        continue;
      }
      if nextcnt == 0 && candi == target {
        res += prop;
        continue;
      }

      next.iter().for_each(|&x| {
        if !visit.contains(&x) {
          let mut newvisit = visit.clone();
          newvisit.insert(x);
          queue.push_back((x, newvisit, step + 1, prop / nextcnt as f64));
        }
      });
    }

    res
  }
}

fn main() {
  println!(
    "{}",
    Solution::frog_position(
      7,
      vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 7],
        vec![2, 4],
        vec![2, 6],
        vec![3, 5]
      ],
      2,
      4
    )
  );
  println!(
    "{}",
    Solution::frog_position(
      7,
      vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 7],
        vec![2, 4],
        vec![2, 6],
        vec![3, 5]
      ],
      1,
      7
    )
  );
  println!(
    "{}",
    Solution::frog_position(
      8,
      vec![
        vec![2, 1],
        vec![3, 2],
        vec![4, 1],
        vec![5, 1],
        vec![6, 4],
        vec![7, 1],
        vec![8, 7]
      ],
      7,
      7
    )
  );
}
