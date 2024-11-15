impl Solution {
  pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
    let rs = grid.len();
    let cs = grid[0].len();

    (0..rs)
      .fold(0, |aggr, ridx| {
        aggr
          + (0..cs / 2).fold(0, |sum, cidx| {
            sum
              + if grid[ridx][cidx] != grid[ridx][cs - 1 - cidx] {
                1
              } else {
                0
              }
          })
      })
      .min((0..cs).fold(0, |aggr, cidx| {
        aggr
          + (0..rs / 2).fold(0, |sum, ridx| {
            sum
              + if grid[ridx][cidx] != grid[rs - 1 - ridx][cidx] {
                1
              } else {
                0
              }
          })
      }))
  }
}
