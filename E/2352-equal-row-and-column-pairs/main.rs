struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp: HashMap<String, i32> = HashMap::new();

    grid.iter().for_each(|row| {
      let mut s: String = "".to_string();
      row.iter().for_each(|v| {
        s.push_str(format!("{:06}", v).as_str());
      });
      dp.entry(s).and_modify(|x| *x += 1).or_insert(1);
    });
    // println!("{:?}", dp);
    let mut res: i32 = 0;
    (0..grid.len()).for_each(|c_idx| {
      let mut s: String = "".to_string();
      (0..grid.len()).for_each(|r_idx| {
        s.push_str(format!("{:06}", grid[r_idx][c_idx]).as_str());
      });
      if dp.contains_key(&s) {
        res += dp.get(&s).unwrap();
      }
    });
    res
  }
}
