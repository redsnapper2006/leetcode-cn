impl Solution {
  pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
    let cords: Vec<Vec<(i32, i32)>> = vec![
      vec![],
      vec![(0, -1), (0, 1)],
      vec![(-1, 0), (1, 0)],
      vec![(0, -1), (1, 0)],
      vec![(0, 1), (1, 0)],
      vec![(0, -1), (-1, 0)],
      vec![(0, 1), (-1, 0)],
    ];

    fn valid(
      cr: i32, cc: i32, nr: i32, nc: i32, cords: &Vec<Vec<(i32, i32)>>, grid: &Vec<Vec<i32>>, visited: &Vec<Vec<bool>>,
    ) -> bool {
      if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 || visited[nr as usize][nc as usize]
      {
        return false;
      }

      for cord in cords[grid[nr as usize][nc as usize] as usize].iter() {
        if nr + cord.0 == cr && nc + cord.1 == cc {
          return true;
        }
      }
      false
    }

    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    fn dfs(r: i32, c: i32, cords: &Vec<Vec<(i32, i32)>>, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> bool {
      if r == grid.len() as i32 - 1 && c == grid[0].len() as i32 - 1 {
        return true;
      }

      visited[r as usize][c as usize] = true;
      for cord in cords[grid[r as usize][c as usize] as usize].iter() {
        if valid(r, c, r + cord.0, c + cord.1, cords, grid, visited)
          && dfs(r + cord.0, c + cord.1, cords, grid, visited)
        {
          return true;
        }
      }
      visited[r as usize][c as usize] = false;
      false
    }
    dfs(0, 0, &cords, &grid, &mut visited)
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::has_valid_path(vec![vec![1, 2, 1], vec![1, 2, 1]]));
  println!("{}", Solution::has_valid_path(vec![vec![2, 4, 3], vec![6, 5, 2]]));
}
