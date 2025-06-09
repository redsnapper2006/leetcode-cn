impl Solution {
  pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let r = grid.len() as i32;
    let c = grid[0].len() as i32;
    let mut idx: i32 = 0;
    let mut ans: Vec<i32> = vec![];
    while idx < r * c {
      let nr = idx / c;
      let nc = idx % c;
      if idx % 2 == 0 {
        ans.push(grid[nr as usize][(if nr % 2 == 1 { c - 1 - nc } else { nc }) as usize]);
      }

      idx += 1;
    }
    ans
  }
}
