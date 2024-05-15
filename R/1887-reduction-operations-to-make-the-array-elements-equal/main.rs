use std::collections::HashMap;
impl Solution {
  pub fn reduction_operations(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    nums.iter().for_each(|&v| {
      *m.entry(v).or_default() += 1;
    });
    let mut keys = m.keys().cloned().collect::<Vec<i32>>();
    keys.sort_unstable();

    let mut ans: i32 = 0;
    (1..keys.len()).for_each(|i| {
      let cnt = m.get(&keys[i]).unwrap();
      ans += *cnt * i as i32;
    });

    ans
  }
}
