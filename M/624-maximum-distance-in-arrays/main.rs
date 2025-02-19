struct Solution {}

impl Solution {
  pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut first: Vec<(i32, usize)> = vec![];
    let mut last: Vec<(i32, usize)> = vec![];
    arrays.iter().enumerate().for_each(|(idx, arr)| {
      first.push((arr[0], idx));
      last.push((arr[arr.len() - 1], idx));
    });

    first.sort_unstable_by(|x, y| {
      if x.0 == y.0 {
        return x.1.cmp(&y.1);
      }
      x.0.cmp(&y.0)
    });
    last.sort_unstable_by(|x, y| {
      if x.0 == y.0 {
        return x.1.cmp(&y.1);
      }
      x.0.cmp(&y.0)
    });

    if last[last.len() - 1].1 != first[0].1 {
      last[last.len() - 1].0 - first[0].0
    } else {
      (last[last.len() - 2].0 - first[0].0).max(last[last.len() - 1].0 - first[1].0)
    }
  }
}
