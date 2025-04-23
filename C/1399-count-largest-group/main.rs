use std::collections::HashMap;
impl Solution {
  pub fn count_largest_group(n: i32) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    (1..=n).for_each(|v| {
      let mut v = v;
      let mut sum: i32 = 0;
      while v > 0 {
        sum += v % 10;
        v /= 10;
      }
      m.entry(sum).and_modify(|x| *x += 1).or_insert(1);
    });

    let mut max: i32 = 0;
    let mut cnt: i32 = 0;
    for (_, v) in m {
      if v > max {
        max = v;
        cnt = 1;
      } else if v == max {
        cnt += 1;
      }
    }
    cnt
  }
}
