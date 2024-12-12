impl Solution {
  pub fn min_element(nums: Vec<i32>) -> i32 {
    *nums
      .iter()
      .map(|&num| {
        let mut n = num;
        let mut sum: i32 = 0;
        while n > 0 {
          sum += n % 10;
          n /= 10;
        }
        sum
      })
      .collect::<Vec<i32>>()
      .iter()
      .min()
      .unwrap()
  }
}
