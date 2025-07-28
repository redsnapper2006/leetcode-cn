use std::collections::HashMap;

impl Solution {
  pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(0, 1);
    let mut sum: i32 = 0;
    nums.iter().for_each(|&n| {
      sum |= n;
      let mut b: HashMap<i32, i32> = HashMap::new();
      for (k, v) in m.iter() {
        b.entry(k | n).and_modify(|x| *x += v).or_insert(*v);
      }
      for (k, v) in b.iter() {
        m.entry(*k).and_modify(|x| *x += v).or_insert(*v);
      }
    });
    *m.get(&sum).unwrap()
  }
}
