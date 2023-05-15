struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    let mut m: HashMap<String, i32> = HashMap::new();
    let mut vv: Vec<(String, String)> = Vec::new();
    matrix.iter().for_each(|row| {
      let mut s: Vec<u8> = Vec::new();
      let mut r_s: Vec<u8> = Vec::new();
      row.iter().for_each(|&v| {
        s.push('0' as u8 + v as u8);
        r_s.push('1' as u8 - v as u8);
      });
      let ss = String::from_utf8(s).unwrap();
      let r_ss = String::from_utf8(r_s).unwrap();
      vv.push((ss.clone(), r_ss));
      m.entry(ss).and_modify(|c| *c += 1).or_insert(1);
    });

    *vv
      .into_iter()
      .map(|(ss, r_ss)| *m.entry(ss).or_insert(0) + *m.entry(r_ss).or_insert(0))
      .collect::<Vec<i32>>()
      .iter()
      .max()
      .unwrap() as i32
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]])
  );
}
