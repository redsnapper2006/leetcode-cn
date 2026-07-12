use std::collections::HashMap;

impl Solution {
  pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort_unstable();
    sorted_arr.dedup();

    let mut m: HashMap<i32, usize> = HashMap::new();
    sorted_arr.iter().enumerate().for_each(|(idx, v)| {
      m.insert(*v, idx);
    });

    arr.iter().map(|v| *m.get(v).unwrap() as i32 + 1).collect::<Vec<i32>>()
  }
}
