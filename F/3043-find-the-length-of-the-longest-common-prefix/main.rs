use std::collections::HashMap;

impl Solution {
  pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut dm: HashMap<i32, i32> = HashMap::new();
    arr1.iter().for_each(|&v| {
      let mut vv = v;
      let mut l = v.checked_ilog10().unwrap_or(0) as i32 + 1;
      while vv > 0 {
        dm.insert(vv, l);
        vv /= 10;
        l -= 1;
      }
    });

    let mut ans: i32 = 0;
    arr2.iter().for_each(|&v| {
      let mut vv = v;
      let mut l = v.checked_ilog10().unwrap_or(0) as i32 + 1;
      while vv > 0 {
        if dm.contains_key(&vv) {
          ans = (*dm.get(&vv).unwrap()).max(ans);
          return;
        }
        vv /= 10;
      }
    });
    ans
  }
}
