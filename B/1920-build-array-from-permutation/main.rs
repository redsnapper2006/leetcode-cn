impl Solution {
  pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
    const BASE: i32 = 1000;
    (0..nums.len()).for_each(|idx| {
      nums[idx] += (nums[(nums[idx] % BASE) as usize] % BASE) * BASE;
    });
    (0..nums.len()).for_each(|idx| {
      nums[idx] /= BASE;
    });
    nums
  }
}

struct Solution {}

fn main() {
  println!("{:?}", Solution::build_array(vec![0, 2, 1, 5, 3, 4]));
  println!("{:?}", Solution::build_array(vec![5, 0, 1, 2, 3, 4]));
}
