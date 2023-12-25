struct Solution {}

impl Solution {
  pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    let remain = tomato_slices - cheese_slices * 2;
    if remain % 2 == 1 || remain < 0 || remain / 2 > cheese_slices {
      return vec![];
    }
    vec![remain / 2, cheese_slices - remain / 2]
  }
}
