struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for item in items1 {
      m.insert(item[0], item[1]);
    }

    for item in items2 {
      if m.contains_key(&item[0]) {
        *(m.get_mut(&item[0]).unwrap()) += item[1];
      } else {
        m.insert(item[0], item[1]);
      }
    }

    let mut keys: Vec<i32> = m.keys().map(|x| *x).collect();
    keys.sort();
    let mut ret: Vec<Vec<i32>> = Vec::new();
    for k in keys {
      ret.push(vec![k, *(m.get(&k).unwrap())]);
    }
    ret
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::merge_similar_items(vec![vec![1, 2]], vec![vec![2, 3]])
  );
}
