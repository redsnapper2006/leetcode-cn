struct Solution {}

impl Solution {
  pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let total: i64 = nums.iter().map(|x| *x as i64).sum();

    let mut ans: i32 = 0;
    let mut sum: i64 = 0;
    (0..nums.len() - 1).for_each(|idx| {
      sum += nums[idx] as i64;
      if sum >= total - sum {
        ans += 1;
      }
    });
    ans
  }
}

fn main() {
  println!("{}", Solution::ways_to_split_array(vec![10, 4, -8, 7]));
  println!("{}", Solution::ways_to_split_array(vec![2, 3, 1, 0]));
}
