impl Solution {
  pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
    let s = nums.len();
    let l = nums
      .into_iter()
      .filter(|x| *x % 2 == 0)
      .collect::<Vec<i32>>()
      .len();
    let mut a = vec![0; l];
    a.append(&mut vec![1; s - l]);
    a
  }
}
