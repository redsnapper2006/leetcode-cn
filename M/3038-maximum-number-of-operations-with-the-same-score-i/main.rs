struct Solution {}

impl Solution {
  pub fn max_operations(nums: Vec<i32>) -> i32 {
    let v = nums
      .chunks_exact(2)
      .map(|v| v.iter().sum())
      .collect::<Vec<i32>>();

    let base = v[0];
    let mut cnt: i32 = 1;
    let mut idx: usize = 1;
    while idx < v.len() {
      if v[idx] == base {
        cnt += 1;
      } else {
        break;
      }
      idx += 1;
    }
    cnt
  }
}

fn main() {
  println!("{}", Solution::max_operations(vec![3, 2, 1, 4, 5]));
}
