impl Solution {
  pub fn max_length(nums: Vec<i32>) -> i32 {
    pub fn gcd(m: i32, n: i32) -> i32 {
      if m > n {
        return gcd(m - n, n);
      } else if m < n {
        return gcd(n - m, m);
      }
      m
    }

    let mut ans: usize = 2;
    let mut prod: i32 = 1;
    let mut start: usize = 0;
    (0..nums.len()).for_each(|end| {
      while gcd(prod, nums[end]) > 1 {
        prod /= nums[start];
        start += 1;
      }
      prod *= nums[end];
      ans = ans.max(end - start + 1);
    });
    ans as i32
  }
}
