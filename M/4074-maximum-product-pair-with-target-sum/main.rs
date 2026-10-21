use std::collections::HashMap;

impl Solution {
  pub fn max_product_pair(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, usize> = HashMap::new();
    let mut mx = i32::MIN;
    let mut ans = vec![-1i32, -1i32];
    for i in 0..nums.len() {
      let p = target - nums[i];
      if nums[i] != p && m.contains_key(&p) && mx < nums[i] * p {
        mx = nums[i] * p;
        let pair = *m.get(&p).unwrap() as i32;
        let (mut p1, mut p2) = (i as i32, pair);
        if nums[i] < p {
          p1 = pair;
          p2 = i as i32;
        }
        ans = vec![p1, p2];
      }
      m.insert(nums[i], i);
    }
    ans
  }
}
