struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
    let mut tasks = tasks
      .into_iter()
      .enumerate()
      .collect::<Vec<(usize, Vec<i32>)>>();
    tasks.sort_by(|(ia, a), (ib, b)| {
      if a[0] == b[0] {
        ia.cmp(ib)
      } else {
        a[0].cmp(&b[0])
      }
    });
    let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

    let mut res: Vec<i32> = Vec::new();
    let mut end: i32 = 0;

    let mut idx: usize = 0;

    while idx < tasks.len() || min_heap.len() > 0 {
      if min_heap.len() == 0 || idx < tasks.len() && end >= tasks[idx].1[0] {
        if idx < tasks.len() && end < tasks[idx].1[0] {
          end = tasks[idx].1[0];
        }

        while idx < tasks.len() && tasks[idx].1[0] <= end {
          min_heap.push(Reverse((tasks[idx].1[1], tasks[idx].0)));
          idx += 1;
        }
      }

      match min_heap.pop() {
        Some(Reverse((n_end, n_idx))) => {
          res.push(n_idx as i32);
          end += n_end;
        }
        None => {}
      }
    }
    res
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]])
  );

  println!(
    "{:?}",
    Solution::get_order(vec![
      vec![7, 10],
      vec![7, 12],
      vec![7, 5],
      vec![7, 4],
      vec![7, 2]
    ])
  );

  println!(
    "{:?}",
    Solution::get_order(vec![
      vec![19, 13],
      vec![16, 9],
      vec![21, 10],
      vec![32, 25],
      vec![37, 4],
      vec![49, 24],
      vec![2, 15],
      vec![38, 41],
      vec![37, 34],
      vec![33, 6],
      vec![45, 4],
      vec![18, 18],
      vec![46, 39],
      vec![12, 24]
    ])
  );
}
