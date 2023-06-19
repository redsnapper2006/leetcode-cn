struct Solution {}

impl Solution {
  pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut one_1: i32 = 1000000;
    let mut one_2: i32 = 1000000;
    let mut two_1: i32 = 1000000;
    let mut two_2: i32 = 1000000;

    let mut sum: i32 = 0;
    nums.iter().for_each(|&v| {
      sum += v;

      if v % 3 == 1 {
        if v < one_1 {
          one_2 = one_1;
          one_1 = v;
        } else if v < one_2 {
          one_2 = v;
        }
      }
      if v % 3 == 2 {
        if v < two_1 {
          two_2 = two_1;
          two_1 = v;
        } else if v < two_2 {
          two_2 = v;
        }
      }
    });

    match (sum % 3, one_1 + one_2 < two_1, two_1 + two_2 < one_1) {
      (1, _, true) => sum - two_1 - two_2,
      (1, _, false) => sum - one_1,
      (2, true, _) => sum - one_1 - one_2,
      (2, false, _) => sum - two_1,
      (_, _, _) => sum,
    }
  }
}
