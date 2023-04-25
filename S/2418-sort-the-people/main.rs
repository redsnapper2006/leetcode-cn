struct Solution {}

impl Solution {
  pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut a = names
      .into_iter()
      .zip(heights.into_iter())
      .collect::<Vec<(String, i32)>>();
    a.sort_by(|x, y| x.1.cmp(&y.1).reverse());
    a.into_iter().map(|v| v.0).collect::<Vec<String>>()
  }
}
