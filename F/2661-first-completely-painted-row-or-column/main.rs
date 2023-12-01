struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let mut buf: HashMap<i32, (usize, usize)> = HashMap::new();

    (0..mat.len()).for_each(|row| {
      (0..mat[0].len()).for_each(|col| {
        buf.insert(mat[row][col], (row, col));
      });
    });

    let mut row: Vec<i32> = vec![0; mat.len()];
    let mut col: Vec<i32> = vec![0; mat[0].len()];
    let mut idx: usize = 0;
    while idx < arr.len() {
      let (r, c) = buf.get(&arr[idx]).unwrap();

      row[*r] += 1;
      col[*c] += 1;
      if row[*r] == mat[0].len() as i32 || col[*c] == mat.len() as i32 {
        return idx as i32;
      }
      idx += 1;
    }
    0
  }
}

fn main() {
  println!(
    "{}",
    Solution::first_complete_index(vec![1, 4, 5, 2, 6, 3], vec![vec![4, 3, 5], vec![1, 2, 6]])
  );
}
