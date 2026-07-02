use std::collections::BinaryHeap;

impl Solution {
  pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
    let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    heap.push((health - grid[0][0], 0, 0));
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    visited[0][0] = true;

    while let Some((h, r, c)) = heap.pop() {
      if r == grid.len() as i32 - 1 && c == grid[0].len() as i32 - 1 {
        return true;
      }
      for (rs, cs) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (nr, nc) = (r + rs, c + cs);
        if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
          continue;
        }
        let (nr, nc) = (nr as usize, nc as usize);
        if h - grid[nr][nc] <= 0 || visited[nr][nc] {
          continue;
        }
        heap.push((h - grid[nr][nc], nr as i32, nc as i32));
        visited[nr][nc] = true;
      }
    }
    false
  }
}
