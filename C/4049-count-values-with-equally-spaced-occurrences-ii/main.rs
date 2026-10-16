use std::collections::HashMap;
impl Solution {
  pub fn count_special_integers(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, (i32, usize, usize, bool)> = HashMap::new();

    for i in 0..nums.len() {
      let v = m.entry(nums[i]).or_insert((0, 0, 0, true));
      if v.0 == 0 || v.0 == 1 || v.3 && i - v.1 == v.2 {
        v.0 += 1;
        v.2 = i - v.1;
        v.1 = i;
      } else {
        v.3 = false;
      }
    }

    let mut ans = 0i32;
    for (_, v) in &m {
      if v.3 && v.0 >= 3 {
        ans += 1;
      }
    }

    ans
  }
}
