struct Solution {}

impl Solution {
  pub fn count_ways(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ans: i32 = 0;
    if nums[0] > 0 {
      ans += 1;
    }
    if nums[nums.len() - 1] < nums.len() as i32 {
      ans += 1;
    }

    let mut idx: usize = 0;
    while idx < nums.len() {
      let c = idx as i32 + 1;
      if c > nums[idx] && c < nums.len() as i32  && c < nums[idx + 1] {
        ans += 1;
      }

      idx += 1;
    }

    ans
  }
}

fn main() {
  println!("{}", Solution::count_ways(vec![1, 1]));
  println!("{}", Solution::count_ways(vec![6, 0, 3, 3, 6, 7, 2, 7]));
}
