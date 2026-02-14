impl Solution {
  pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let query_glass = query_glass as usize;
    let query_row = query_row as usize;
    let mut row: Vec<f64> = vec![0.0; query_row + 1];
    row[0] = poured as f64;
    for i in 0..query_row {
      (0..i + 1).rev().for_each(|j| {
        if row[j] > 1.0 {
          row[j + 1] += (row[j] - 1.0) / 2.0;
          row[j] = (row[j] - 1.0) / 2.0;
        } else {
          row[j] = 0.0;
        }
      });
    }

    row[query_glass].min(1.0)
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::champagne_tower(1, 1, 1));
}
