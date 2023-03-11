struct Solution {}

impl Solution {
  pub fn left_rigth_difference(nums: Vec<i32>) -> Vec<i32> {
    let mut left: Vec<i32> = vec![0; nums.len()];
    let mut right: Vec<i32> = vec![0; nums.len()];

    let mut ls: i32 = 0;
    let mut rs: i32 = 0;
    (0..nums.len()).for_each(|idx| {
      match idx {
        0 => {
          left[idx] = 0;
          right[nums.len() - 1 - idx] = 0;
        }
        _ => {
          left[idx] = ls;
          right[nums.len() - 1 - idx] = rs;
        }
      }

      ls += nums[idx];
      rs += nums[nums.len() - 1 - idx];
    });

    left.iter().zip(right).map(|(l,r)| (l-r).abs()).collect::<Vec<i32>>()
  }
}
