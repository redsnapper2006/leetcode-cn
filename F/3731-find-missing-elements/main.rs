impl Solution {
  pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
    let mn = nums.iter().min().unwrap();
    let mx = nums.iter().max().unwrap();
    let mut dp: Vec<i32> = vec![0; (mx - mn) as usize + 1];
    nums.iter().for_each(|v| {
      dp[(v - mn) as usize] = 1;
    });
    let mut ans: Vec<i32> = vec![];
    (mn..=mx).for_each(|x| {
      if dp[(x - mn) as usize] == 0 {
        ans.push(x);
      }
    });
    ans
  }
}
