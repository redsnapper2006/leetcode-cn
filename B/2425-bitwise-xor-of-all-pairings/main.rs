struct Solution {}

impl Solution {
  pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let x1 = nums1.iter().fold(0, |acc, x| acc ^ x);
    let x2 = nums2.iter().fold(0, |acc, x| acc ^ x);

    match (nums1.len() % 2, nums2.len() % 2) {
      (0,0) => 0,
      (0,1) => x1,
      (1,0) => x2,
      (_,_) => x1^x2
    }
  }
}
