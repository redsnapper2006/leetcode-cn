use std::collections::HashMap;

impl Solution {
  pub fn longest_balanced(nums: Vec<i32>) -> i32 {
    let mut even: HashMap<i32, i32> = HashMap::new();
    let mut odd: HashMap<i32, i32> = HashMap::new();
    let mut ans: usize = 0;
    for i in 1..nums.len() {
      even.clear();
      odd.clear();
      if nums[i] % 2 == 0 {
        even.insert(nums[i], 1);
      } else {
        odd.insert(nums[i], 1);
      }
      for j in (0..i).rev() {
        if nums[j] % 2 == 0 {
          *even.entry(nums[j]).or_insert(0) += 1;
        } else {
          *odd.entry(nums[j]).or_insert(0) += 1;
        }
        if even.len() == odd.len() {
          ans = ans.max(i - j + 1);
        }
      }
    }
    ans as _
  }
}
