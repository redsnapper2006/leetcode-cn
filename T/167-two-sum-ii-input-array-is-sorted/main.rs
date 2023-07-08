struct Solution {}

impl Solution {
  pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut start: usize = 0;
    let mut end: usize = numbers.len() - 1;
    while start < end {
      let sum = numbers[start] + numbers[end];
      if sum == target {
        break;
      } else if sum > target {
        end -= 1;
      } else {
        start += 1;
      }
    }
    vec![start as i32 + 1, end as i32 + 1]
  }
}
