struct Solution {}

impl Solution {
  pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let mut even: i64 = nums[0] as i64;
    let mut odd: i64 = 0;
    (1..nums.len()).for_each(|n| {
      even = even.max(odd + nums[n] as i64);
      odd = odd.max(even - nums[n] as i64);
    });
    even.max(odd)
  }

  pub fn max_alternating_sum2(nums: Vec<i32>) -> i64 {
    let mut buf: Vec<i64> = Vec::new();
    let mut is_peek: bool = true;
    nums.iter().enumerate().for_each(|(idx, &num)| {
      let mut l: i32 = 0;
      if idx > 0 {
        l = nums[idx - 1];
      }
      let mut r: i32 = 0;
      if idx < nums.len() - 1 {
        r = nums[idx + 1];
      }
      if is_peek && num >= l && num >= r {
        is_peek = false;
        buf.push(num as i64);
      }
      if !is_peek && num <= l && num <= r {
        is_peek = true;
        buf.push((-num) as i64);
      }
    });
    // println!("{:?}",buf);
    if buf.len() % 2 == 0 {
      buf.pop();
    }
    buf.iter().sum::<i64>()
  }
}
