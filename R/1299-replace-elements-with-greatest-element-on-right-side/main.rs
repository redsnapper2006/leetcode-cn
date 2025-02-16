struct Solution {}

impl Solution {
  pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    arr
      .iter()
      .rev()
      .fold((Vec::<i32>::new(), -1), |(mut ans, max), &v| {
        ans.push(max);
        (ans, max.max(v))
      })
      .0
      .into_iter()
      .rev()
      .collect::<Vec<i32>>()
  }
}
