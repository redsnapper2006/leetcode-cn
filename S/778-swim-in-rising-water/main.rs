struct Solution {}

impl Solution {
  pub fn dfs(
    grid: &Vec<Vec<i32>>,
    buf: &mut Vec<Vec<i32>>,
    r: usize,
    c: usize,
    max: i32,
    visit: &mut Vec<[usize; 2]>,
  ) {
    if r > 0 && buf[r - 1][c] == -1 {
      if grid[r - 1][c] <= max {
        buf[r - 1][c] = max;
        visit.push([r - 1, c]);
        Solution::dfs(grid, buf, r - 1, c, max, visit);
      }
    }
    if c > 0 && buf[r][c - 1] == -1 {
      if grid[r][c - 1] <= max {
        buf[r][c - 1] = max;
        visit.push([r, c - 1]);
        Solution::dfs(grid, buf, r, c - 1, max, visit);
      }
    }
    if r < grid.len() - 1 && buf[r + 1][c] == -1 {
      if grid[r + 1][c] <= max {
        buf[r + 1][c] = max;
        visit.push([r + 1, c]);
        Solution::dfs(grid, buf, r + 1, c, max, visit);
      }
    }
    if c < grid[0].len() - 1 && buf[r][c + 1] == -1 {
      if grid[r][c + 1] <= max {
        buf[r][c + 1] = max;
        visit.push([r, c + 1]);
        Solution::dfs(grid, buf, r, c + 1, max, visit);
      }
    }
  }
  pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];
    // for row in buf {
    //     println!("new line");
    //     for j in &row {
    //         println!("{}", j);
    //     }
    // }
    let mut visit: Vec<[usize; 2]> = vec![[0; 2]];
    buf[0][0] = grid[0][0];
    Solution::dfs(&grid, &mut buf, 0, 0, grid[0][0], &mut visit);
    let mut ret: i32 = -1;
    let mut isFound: bool = false;
    while !isFound {
      let mut min: i32 = 10000;
      let mut t: Vec<[usize; 2]> = Vec::new();
      for i in 0..visit.len() {
        let candi = visit[i];
        let r = candi[0];
        let c = candi[1];
        if r > 0 && buf[r - 1][c] == -1 {
          if grid[r - 1][c] < min {
            min = grid[r - 1][c];
            t = Vec::new();
            t.push([r - 1, c]);
          } else if grid[r - 1][c] == min {
            t.push([r - 1, c]);
          }
        }
        if c > 0 && buf[r][c - 1] == -1 {
          if grid[r][c - 1] < min {
            min = grid[r][c - 1];
            t = Vec::new();
            t.push([r, c - 1]);
          } else if grid[r][c - 1] == min {
            t.push([r, c - 1]);
          }
        }
        if r < grid.len() - 1 && buf[r + 1][c] == -1 {
          if grid[r + 1][c] < min {
            min = grid[r + 1][c];
            t = Vec::new();
            t.push([r + 1, c]);
          } else if grid[r + 1][c] == min {
            t.push([r + 1, c]);
          }
        }
        if c < grid[0].len() - 1 && buf[r][c + 1] == -1 {
          if grid[r][c + 1] < min {
            min = grid[r][c + 1];
            t = Vec::new();
            t.push([r, c + 1]);
          } else if grid[r][c + 1] == min {
            t.push([r, c + 1]);
          }
        }
      }
      if t.len() == 0 {
        break;
      }
      for i in 0..t.len() {
        let candi = t[i];
        let r = candi[0];
        let c = candi[1];
        if r == grid.len() - 1 && c == grid[0].len() - 1 {
          ret = min;
          isFound = true;
          break;
        }
        buf[r][c] = min;
        visit.push(candi);
        Solution::dfs(&grid, &mut buf, r, c, min, &mut visit);
      }
    }
    if ret == -1 {
      ret = buf[grid.len() - 1][grid[0].len() - 1]
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::swim_in_water(vec![vec![0; 5]; 5]));
}
