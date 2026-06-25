impl Solution {
  pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
    let mut pre: Vec<i64> = vec![0; 2 * nums.len() + 1];
    pre[nums.len()] = 1;

    let mut cnt: usize = nums.len();
    let mut sum: i64 = 0;
    let mut ans: i64 = 0;
    nums.iter().for_each(|&v| {
      if v == target {
        sum += pre[cnt];
        cnt += 1;
        pre[cnt] += 1;
      } else {
        cnt -= 1;
        sum -= pre[cnt];
        pre[cnt] += 1;
      }
      ans += sum;
    });

    ans
  }
}
