struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut edges: Vec<Vec<(i32, i32)>> = vec![Vec::new(); n as usize + 1];
    times.iter().for_each(|time| {
      edges[time[0] as usize].push((time[1], time[2]));
    });

    let mut dis: Vec<i32> = vec![-1; n as usize + 1];
    let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    heap.push(Reverse((0, k)));
    dis[k as usize] = 0;

    while heap.len() > 0 {
      let e = heap.pop().unwrap().0;
      edges[e.1 as usize].iter().for_each(|&(ne, nd)| {
        if dis[ne as usize] == -1 || dis[ne as usize] > e.0 + nd {
          dis[ne as usize] = e.0 + nd;
          heap.push(Reverse((e.0 + nd, ne)));
        }
      });
    }

    let mut ans: i32 = -1;
    let mut idx: usize = 1;
    while idx <= n as usize {
      if dis[idx] == -1 {
        return -1;
      }

      if ans == -1 || ans < dis[idx] {
        ans = dis[idx];
      }
      idx += 1;
    }

    ans
  }
}
