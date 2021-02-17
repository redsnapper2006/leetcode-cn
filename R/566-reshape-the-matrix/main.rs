struct Solution {}

impl Solution {
  pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let aggr = nums.concat();
    if aggr.len() != (r * c) as usize {
      return nums;
    }
    let ret = aggr.chunks(c as usize).map(|x| x.to_vec()).collect();
    ret
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4)
  );
}
