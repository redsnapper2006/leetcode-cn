struct Solution {}

use std::collections::BTreeMap;
impl Solution {
  pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
    let mut tm1: BTreeMap<i32, i32> = BTreeMap::new();
    let mut tm2: BTreeMap<i32, i32> = BTreeMap::new();
    points.iter().for_each(|point| {
      let x = point[0];
      let y = point[1];
      tm1.entry(x + y).and_modify(|v| *v += 1).or_insert(1);
      tm2.entry(x - y).and_modify(|v| *v += 1).or_insert(1);
    });

    let mut ans: i32 = std::i32::MAX;
    points.iter().for_each(|point| {
      let x = point[0];
      let y = point[1];

      tm1.entry(x + y).and_modify(|v| *v -= 1);
      if *tm1.get(&(x + y)).unwrap() == 0 {
        tm1.remove(&(x + y));
      }
      tm2.entry(x - y).and_modify(|v| *v -= 1);
      if *tm2.get(&(x - y)).unwrap() == 0 {
        tm2.remove(&(x - y));
      }

      let max1 = tm1.iter().rev().next().unwrap().0 - tm1.iter().next().unwrap().0;
      let max2 = tm2.iter().rev().next().unwrap().0 - tm2.iter().next().unwrap().0;
      ans = ans.min(max1.max(max2));
      tm1.entry(x + y).and_modify(|v| *v += 1).or_insert(1);
      tm2.entry(x - y).and_modify(|v| *v += 1).or_insert(1);
    });
    ans
  }
}
