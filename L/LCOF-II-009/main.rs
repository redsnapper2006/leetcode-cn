struct Solution {}

impl Solution {
  pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut times: i64 = 1;
    let mut res: i32 = 0;
    let mut start: usize = 0;
    let mut end: usize = 0;
    while end < nums.len() {
      times *= nums[end] as i64;
      if times >= k as i64 {
        while times >= k as i64 && start <= end {
          times /= nums[start] as i64;
          start += 1;
        }
      }
      res += (end - start + 1) as i32;
      end += 1;
    }
    res
  }
}

fn main() {
  println!(
    "{}",
    Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100)
  );
}
