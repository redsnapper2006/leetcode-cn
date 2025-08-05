use std::collections::BTreeMap;

use std::ops::Bound::Included;
impl Solution {
  pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    let mut tm: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    for i in 0..baskets.len() {
      tm.entry(baskets[i])
        .and_modify(|ids| ids.push(i))
        .or_insert(vec![i]);
    }

    let mut cnt: i32 = 0;
    fruits.iter().for_each(|fruit| {
      let mut kk: i32 = -1;
      let mut off: usize = baskets.len() + 1;
      for (&key, value) in tm.range((Included(fruit), Included(&10000))) {
        if kk == -1 || value[0] < off {
          off = value[0];
          kk = key;
        }
      }
      if kk != -1 {
        tm.get_mut(&kk).unwrap().remove(0);
        if tm.get(&kk).unwrap().len() == 0 {
          tm.remove(&kk);
        }
      } else {
        cnt += 1;
      }
    });
    cnt
  }
}
