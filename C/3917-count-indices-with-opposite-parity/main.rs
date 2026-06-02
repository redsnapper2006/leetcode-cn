impl Solution {
  pub fn count_opposite_parity(nums: Vec<i32>) -> Vec<i32> {
    let (odd, even) = nums.iter().fold((0, 0), |(odd, even), v| (odd + v % 2, even + (v + 1) % 2));
    nums
      .iter()
      .fold((vec![], 0, 0), |(mut ans, oc, ec), v| {
        ans.push(if v % 2 == 1 { even - ec } else { odd - oc });
        (ans, oc + v % 2, ec + (v + 1) % 2)
      })
      .0
  }
}
