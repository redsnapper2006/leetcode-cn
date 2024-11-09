use std::collections::HashMap;

struct NeighborSum {
  grid: Vec<Vec<i32>>,
  am: HashMap<i32, i32>,
  dm: HashMap<i32, i32>,
}

impl NeighborSum {
  fn new(grid: Vec<Vec<i32>>) -> Self {
    let acord: Vec<Vec<i32>> = vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]];
    let dcord: Vec<Vec<i32>> = vec![vec![-1, -1], vec![-1, 1], vec![1, -1], vec![1, 1]];

    let mut am: HashMap<i32, i32> = HashMap::new();
    let mut dm: HashMap<i32, i32> = HashMap::new();

    (0..grid.len() as i32).for_each(|r| {
      (0..grid[0].len() as i32).for_each(|c| {
        let mut a: i32 = 0;
        acord.iter().for_each(|ac| {
          let nr = r + ac[0];
          let nc = c + ac[1];
          if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
            return;
          }
          a += grid[nr as usize][nc as usize];
        });
        let mut d: i32 = 0;
        dcord.iter().for_each(|dc| {
          let nr = r + dc[0];
          let nc = c + dc[1];
          if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
            return;
          }
          d += grid[nr as usize][nc as usize];
        });
        am.entry(grid[r as usize][c as usize]).or_insert(a);
        dm.entry(grid[r as usize][c as usize]).or_insert(d);
      });
    });

    NeighborSum {
      grid: grid,
      am: am,
      dm: dm,
    }
  }

  fn adjacent_sum(&self, value: i32) -> i32 {
    *self.am.get(&value).unwrap()
  }

  fn diagonal_sum(&self, value: i32) -> i32 {
    *self.dm.get(&value).unwrap()
  }
}
