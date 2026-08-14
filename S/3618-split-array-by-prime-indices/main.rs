impl Solution {
  pub fn split_array(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    // 埃氏筛
    let mut is_prime = vec![true; n];
    if n > 0 {
      is_prime[0] = false;
    }
    if n > 1 {
      is_prime[1] = false;
    }
    let mut i = 2;
    while i * i < n {
      if is_prime[i] {
        let mut j = i * i;
        while j < n {
          is_prime[j] = false;
          j += i;
        }
      }
      i += 1;
    }

    let mut a: i64 = 0;
    for i in 0..nums.len() {
      a += nums[i] as i64 * if is_prime[i] { 1 } else { -1 }
    }

    a.abs()
  }
}
