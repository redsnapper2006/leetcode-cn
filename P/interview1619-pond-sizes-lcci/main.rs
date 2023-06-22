struct Solution {}

impl Solution {
  pub fn dfs(land: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    if i >= land.len() as i32
      || j >= land[0].len() as i32
      || i < 0
      || j < 0
      || land[i as usize][j as usize] != 0
    {
      return 0;
    }
    land[i as usize][j as usize] = -1;
    let mut res = 1;
    res += Self::dfs(land, i + 1, j);
    res += Self::dfs(land, i - 1, j);
    res += Self::dfs(land, i, j + 1);
    res += Self::dfs(land, i, j - 1);
    res += Self::dfs(land, i + 1, j + 1);
    res += Self::dfs(land, i + 1, j - 1);
    res += Self::dfs(land, i - 1, j + 1);
    res += Self::dfs(land, i - 1, j - 1);
    res
  }

  pub fn pond_sizes(land: Vec<Vec<i32>>) -> Vec<i32> {
    let mut land = land;
    let mut res = vec![];
    for i in 0..land.len() {
      for j in 0..land[0].len() {
        if land[i][j] == 0 {
          res.push(Self::dfs(&mut land, i as i32, j as i32));
        }
      }
    }
    res.sort();
    res
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::pond_sizes(vec![
      vec![0, 2, 1, 0],
      vec![0, 1, 0, 1],
      vec![1, 1, 0, 1],
      vec![0, 1, 0, 1]
    ])
  );
}
