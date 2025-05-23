impl Solution {
  pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
    let mut f0: i64 = 0;
    let mut f1: i64 = i64::MIN;
    nums.iter().for_each(|&v| {
      (f0, f1) = (
        (f0 + v as i64).max(f1 + (v ^ k) as i64),
        (f1 + v as i64).max(f0 + (v ^ k) as i64),
      );
    });

    f0
  }
}
