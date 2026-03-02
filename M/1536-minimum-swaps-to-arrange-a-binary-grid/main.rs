impl Solution {
  pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut arr: Vec<i32> = grid
      .iter()
      .map(|g| {
        for i in (0..g.len()).rev() {
          if g[i] == 1 {
            return (g.len() - 1 - i) as i32;
          }
        }
        g.len() as i32
      })
      .collect::<Vec<i32>>();

    let mut ans: i32 = 0;
    for i in (0..n) {
      let mut idx: usize = i;
      while idx < arr.len() && arr[idx] < (n - 1 - i) as i32 {
        idx += 1;
      }
      if idx == arr.len() {
        return -1;
      }
      (i..idx).rev().for_each(|ii| {
        let t = arr[ii + 1];
        arr[ii + 1] = arr[ii];
        arr[ii] = t;
        ans += 1;
      });
    }
    ans
  }
}
