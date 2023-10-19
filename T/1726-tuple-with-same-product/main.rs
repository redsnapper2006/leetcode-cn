struct Solution {}

impl Solution {
  pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let mut map = std::collections::HashMap::new();
    let mut result = 0;
    for i in 0..nums.len() {
      for j in i + 1..nums.len() {
        let product = nums[i] * nums[j];
        let count = map.entry(product).or_insert(0);
        *count += 1;
      }
    }
    for (_, v) in map {
      if v > 1 {
        result += v * (v - 1) * 4;
      }
    }
    result


  }
}
