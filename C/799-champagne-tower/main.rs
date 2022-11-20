struct Solution {}

impl Solution {
  pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut row: Vec<f64> = vec![poured as f64];
    for i in 1..=query_row {
      let mut next_row: Vec<f64> = Vec::new();
      next_row.resize(i as usize + 1, 0.0);
      // println!("{} {:?}",i, next_row);
      for (j, vol) in row.iter().enumerate() {
        if *vol > 1.0 {
          next_row[j] += (*vol - 1.0) / 2.0;
          next_row[j + 1] += (*vol - 1.0) / 2.0;
        }
      }
      row = next_row;
    }
    // println!("{:?}", row);
    if row[query_glass as usize] > 1.0 {
      return 1.0;
    }
    row[query_glass as usize]
  }
}

fn main() {
  println!("{}", Solution::champagne_tower(1, 1, 1));
}
