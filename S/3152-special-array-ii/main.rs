struct Solution {}

impl Solution {
  pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut buf: Vec<usize> = vec![0; nums.len()];
    let mut idx: usize = 1;
    while idx < nums.len() {
      if nums[idx] % 2 == nums[idx - 1] % 2 {
        buf[idx] = idx;
      } else {
        buf[idx] = buf[idx - 1];
      }
      idx += 1;
    }
    let mut ans: Vec<bool> = Vec::new();
    queries.iter().for_each(|query| {
      ans.push(buf[query[1] as usize] <= query[0] as usize);
    });

    ans
  }
}
