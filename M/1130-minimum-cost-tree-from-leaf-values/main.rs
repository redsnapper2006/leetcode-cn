struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    let mut dp: HashMap<usize, HashMap<usize, (i32, i32)>> = HashMap::new();

    fn dfs(
      start: usize,
      end: usize,
      arr: &Vec<i32>,
      dp: &mut HashMap<usize, HashMap<usize, (i32, i32)>>,
    ) -> (i32, i32) {
      if dp.contains_key(&start) && dp.get(&start).unwrap().contains_key(&end) {
        return *dp.get(&start).unwrap().get(&end).unwrap();
      }

      if !dp.contains_key(&start) {
        dp.insert(start, HashMap::new());
      }

      if start == end {
        dp.get_mut(&start).unwrap().insert(end, (arr[start], 0));
        return (arr[start], 0);
      }

      let mut idx: usize = start;
      let mut min: i32 = 1 << 31 - 1;
      let mut sub_max: i32 = 0;
      while idx < end {
        let (l1, l2) = dfs(start, idx, arr, dp);
        let (r1, r2) = dfs(idx + 1, end, arr, dp);
        if min > l1 * r1 + l2 + r2 {
          min = l1 * r1 + l2 + r2;
          sub_max = l1;
          if sub_max < r1 {
            sub_max = r1;
          }
        }
        idx += 1;
      }
      dp.get_mut(&start).unwrap().insert(end, (sub_max, min));
      (sub_max, min)
    }

    let (_, aggr) = dfs(0, arr.len() - 1, &arr, &mut dp);
    aggr
  }
}

fn main() {
  println!("{}", Solution::mct_from_leaf_values(vec![6, 2, 4]));
  println!("{}", Solution::mct_from_leaf_values(vec![11, 4]));
}
