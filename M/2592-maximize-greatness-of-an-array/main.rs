impl Solution {
  pub fn maximize_greatness(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ans : i32 = 0;
    let mut right : usize = 0;
    let mut left : usize = 0;
    while right < nums.len() {
      if nums[right] > nums[left] {
        ans += 1;
        left +=1;
      }
      right += 1;
    }
    ans

  }
}
