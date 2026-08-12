impl Solution {
  pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut m: std::collections::HashMap<i32, i32> =
      std::collections::HashMap::with_capacity(nums.len());
    let mut start: usize = 0;
    let mut ans: usize = 0;
    for (idx, &v) in nums.iter().enumerate() {
      *m.entry(v).or_insert(0) += 1;

      while *m.get(&v).unwrap() > k {
        *m.get_mut(&nums[start]).unwrap() -= 1;
        start += 1;
      }
      ans = ans.max(idx - start + 1);
    }

    ans as i32
  }
}

struct Solution {}
