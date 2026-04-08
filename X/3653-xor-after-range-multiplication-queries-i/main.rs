impl Solution {
  pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    const M: i64 = 1000000007;
    let mut nums = nums.iter().map(|&x| x as i64).collect::<Vec<i64>>();

    queries.iter().for_each(|query| {
      (query[0]..=query[1]).step_by(query[2] as usize).for_each(|idx| {
        let idx = idx as usize;
        nums[idx] = (nums[idx] * query[3] as i64) % M;
      });
    });

    let mut ans = nums[0];
    for i in 1..nums.len() {
      ans ^= nums[i];
    }
    ans as _
  }
}
