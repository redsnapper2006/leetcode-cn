impl Solution {
  pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut bb = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
    bb.sort_by(|x, y| y.1.cmp(&x.1).then_with(|| y.0.cmp(&x.0)));

    let mut bb2 = bb[0..k as usize].to_vec();
    bb2.sort_by(|x, y| x.0.cmp(&y.0));
    bb2.iter().map(|x| x.1).collect::<Vec<i32>>()
  }
}
