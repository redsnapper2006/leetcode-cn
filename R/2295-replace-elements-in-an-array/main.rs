use std::collections::HashMap;
impl Solution {
  pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
    let mut m: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
      m.insert(nums[i], i);
    }
    operations.iter().for_each(|oper| {
      m.insert(oper[1], *m.get(&oper[0]).unwrap());
      m.remove(&oper[0]);
    });
    let mut ans: Vec<i32> = vec![0; nums.len()];
    for i in 0..nums.len() {
      ans[i] = nums[i];
    }
    for (k, v) in m.iter() {
      ans[*v] = *k;
    }
    ans
  }
}
