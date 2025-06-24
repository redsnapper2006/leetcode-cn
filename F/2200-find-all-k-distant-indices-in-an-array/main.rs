impl Solution {
  pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let mut start: i32 = 0;

    let mut ans: Vec<i32> = vec![];
    for (idx, v) in nums.iter().enumerate() {
      if *v == key {
        let idx = idx as i32;
        let end = (idx + k).min(nums.len() as i32 - 1);
        ((idx - k).max(start)..=end).for_each(|v| {
          ans.push(v);
        });
        start = end + 1;
      }
    }
    ans
  }
}
