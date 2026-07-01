use std::collections::{BinaryHeap, VecDeque};
impl Solution {
  pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut distance: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; n];
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    for r in 0..n {
      for c in 0..n {
        if grid[r][c] == 1 {
          q.push_back((r as i32, c as i32, 0));
          distance[r][c] = 0;
        }
      }
    }

    while q.len() > 0 {
      let (r, c, d) = q.pop_front().unwrap();
      for (rs, cs) in vec![(0, -1), (0, 1), (1, 0), (-1, 0)] {
        let nr = r + rs;
        let nc = c + cs;
        if nr < 0
          || nr >= n as i32
          || nc < 0
          || nc >= n as i32
          || distance[nr as usize][nc as usize] <= d + 1
        {
          continue;
        }
        distance[nr as usize][nc as usize] = d + 1;
        q.push_back((nr, nc, d + 1));
      }
    }

    let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    let mut visit: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut mx = distance[0][0].min(distance[n - 1][n - 1]);
    heap.push((distance[0][0], 0, 0));
    visit[0][0] = true;

    while let Some((dis, r, c)) = heap.pop() {
      mx = mx.min(dis);
      if r == n as i32 - 1 && c == n as i32 - 1 {
        break;
      }

      for (rs, cs) in vec![(0, -1), (0, 1), (1, 0), (-1, 0)] {
        let nr = r + rs;
        let nc = c + cs;
        if nr < 0 || nr >= n as i32 || nc < 0 || nc >= n as i32 || visit[nr as usize][nc as usize] {
          continue;
        }
        heap.push((distance[nr as usize][nc as usize], nr, nc));
        visit[nr as usize][nc as usize] = true;
      }
    }

    mx
  }

  pub fn maximum_safeness_factor2(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut distance: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; n];
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    for r in 0..n {
      for c in 0..n {
        if grid[r][c] == 1 {
          q.push_back((r as i32, c as i32, 0));
          distance[r][c] = 0;
        }
      }
    }

    while q.len() > 0 {
      let (r, c, d) = q.pop_front().unwrap();
      for (rs, cs) in vec![(0, -1), (0, 1), (1, 0), (-1, 0)] {
        let nr = r + rs;
        let nc = c + cs;
        if nr < 0
          || nr >= n as i32
          || nc < 0
          || nc >= n as i32
          || distance[nr as usize][nc as usize] <= d + 1
        {
          continue;
        }
        distance[nr as usize][nc as usize] = d + 1;
        q.push_back((nr, nc, d + 1));
      }
    }

    fn find(r: usize, c: usize, set: &mut Vec<Vec<(usize, usize)>>) -> (usize, usize) {
      if set[r][c] == (r, c) {
        return (r, c);
      }
      let (nr, nc) = set[r][c];
      set[r][c] = find(nr, nc, set);
      return set[r][c];
    }
    fn union(r1: usize, c1: usize, r2: usize, c2: usize, set: &mut Vec<Vec<(usize, usize)>>) {
      let (rr1, cc1) = find(r1, c1, set);
      let (rr2, cc2) = find(r2, c2, set);
      if rr1 < rr2 || rr1 == rr2 && cc1 < cc2 {
        set[r2][c2] = (rr1, cc1);
      } else {
        set[r1][c1] = (rr2, cc2);
      }
    }

    let mut start: i32 = 0;
    let mut end: i32 = distance[0][0].min(distance[n - 1][n - 1]);
    let mut uf_set: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); n]; n];
    while start <= end {
      let mid = start + (end - start) / 2;
      for r in 0..n {
        for c in 0..n {
          uf_set[r][c] = (r, c);
        }
      }

      for r in 0..n as i32 {
        for c in 0..n as i32 {
          if distance[r as usize][c as usize] < mid {
            continue;
          }
          for (rs, cs) in vec![(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let nr = r + rs;
            let nc = c + cs;
            if nr < 0
              || nr >= n as i32
              || nc < 0
              || nc >= n as i32
              || distance[nr as usize][nc as usize] < mid
            {
              continue;
            }
            union(
              r as usize,
              c as usize,
              nr as usize,
              nc as usize,
              &mut uf_set,
            );
          }
        }
      }

      if find(0, 0, &mut uf_set) == find(n - 1, n - 1, &mut uf_set) {
        start = mid + 1;
      } else {
        end = mid - 1;
      }
    }

    start - 1
  }
}
