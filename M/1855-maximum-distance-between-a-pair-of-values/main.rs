impl Solution {
  pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    nums1
      .iter()
      .enumerate()
      .fold((0, 0), |(ans, mut idx2), (idx1, &v)| {
        while idx2 < nums2.len() && nums2[idx2] >= v {
          idx2 += 1;
        }
        (
          if idx2 <= idx1 {
            ans
          } else {
            ans.max(idx2 - idx1 - 1)
          },
          idx2,
        )
      })
      .0 as _
  }
}
