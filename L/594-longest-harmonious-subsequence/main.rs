use std::collections::HashMap;
impl Solution {
  pub fn find_lhs(nums: Vec<i32>) -> i32 {
    nums
      .iter()
      .fold((0, HashMap::<i32, i32>::new()), |(max, mut m), &v| {
        m.entry(v).and_modify(|x| *x += 1).or_insert(1);
        m.entry(v - 1).or_insert(0);
        m.entry(v + 1).or_insert(0);
        let vv = (*m.get(&(v + 1)).unwrap()).max((*m.get(&(v - 1)).unwrap()));
        if vv == 0 {
          (max, m)
        } else {
          (max.max(*m.get(&v).unwrap() + vv), m)
        }
      })
      .0
  }
}
