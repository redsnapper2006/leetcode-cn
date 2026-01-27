use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
  pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut em: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();
    let mut cost: Vec<i32> = vec![i32::MAX; n];

    edges.iter().for_each(|edge| {
      em.entry(edge[0] as usize).or_insert(vec![]).push((edge[1] as usize, edge[2]));
      em.entry(edge[1] as usize)
        .or_insert(vec![])
        .push((edge[0] as usize, 2 * edge[2]));
    });
    cost[0] = 0;

    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, 0)));
    while heap.len() > 0 {
      let candi = heap.pop().unwrap().0;
      if !em.contains_key(&candi.1) {
        continue;
      }

      if candi.1 == n - 1 {
        return candi.0;
      }

      for nxt in em.get(&candi.1).unwrap().iter() {
        if cost[nxt.0] <= candi.0 + nxt.1 {
          continue;
        }

        cost[nxt.0] = candi.0 + nxt.1;
        heap.push(Reverse((candi.0 + nxt.1, nxt.0)));
      }
    }

    -1
  }

  pub fn min_cost2(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut em: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();
    let mut cost: Vec<i32> = vec![i32::MAX; n];

    edges.iter().for_each(|edge| {
      em.entry(edge[0] as usize).or_insert(vec![]).push((edge[1] as usize, edge[2]));
      em.entry(edge[1] as usize)
        .or_insert(vec![])
        .push((edge[0] as usize, 2 * edge[2]));
    });
    cost[0] = 0;

    let mut q: VecDeque<(usize, i32)> = VecDeque::new();
    q.push_back((0, 0));
    while q.len() > 0 {
      let candi = q.pop_front().unwrap();
      if !em.contains_key(&candi.0) {
        continue;
      }

      em.get(&candi.0).unwrap().iter().for_each(|&nxt| {
        if cost[nxt.0] <= candi.1 + nxt.1 {
          return;
        }

        cost[nxt.0] = candi.1 + nxt.1;
        q.push_back((nxt.0, candi.1 + nxt.1));
      });
    }

    if cost[n - 1] == i32::MAX {
      -1
    } else {
      cost[n - 1]
    }
  }
}
