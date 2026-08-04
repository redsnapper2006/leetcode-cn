impl Solution {
  pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
    let (mn, mx) = nums.iter().fold((i32::MAX, i32::MIN), |(mn, mx), &v| (mn.min(v), mx.max(v)));
    let mut dp: Vec<bool> = vec![false; (mx - mn) as usize + 1];
    for v in nums.iter() {
      dp[(v - mn) as usize] = true;
    }
    let mut ans: Vec<i32> = vec![];
    for i in mn..=mx {
      if !dp[(i - mn) as usize] {
        ans.push(i);
      }
    }
    ans
  }
}
