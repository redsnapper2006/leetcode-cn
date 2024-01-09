struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    let mut diff_vec: Vec<i64> = Vec::new();
    let mut diff_vec_map: Vec<HashMap<i64, i32>> = Vec::new();
    diff_vec_map.push(HashMap::new());
    (1..nums.len()).for_each(|idx| {
      let d = nums[idx] as i64 - nums[idx - 1] as i64;
      (0..diff_vec.len()).for_each(|i| {
        diff_vec[i] += d;
      });
      diff_vec.push(d);
      let mut m: HashMap<i64, i32> = HashMap::new();
      (0..diff_vec.len()).for_each(|i| {
        let v = diff_vec[i];
        let c = diff_vec_map[i].entry(v).or_insert(0);
        *m.entry(v).or_insert(0) += *c + 1;
        res += *c;
      });
      diff_vec_map.push(m);
    });

    res
  }
}

fn main() {
  println!(
    "{}",
    Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10])
  );

  println!(
    "{}",
    Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7])
  );

  println!(
    "{}",
    Solution::number_of_arithmetic_slices(vec![
      79, 20, 64, 28, 67, 81, 60, 58, 97, 85, 92, 96, 82, 89, 46, 50, 15, 2, 36, 44, 54, 2, 90, 37,
      7, 79, 26, 40, 34, 67, 64, 28, 60, 89, 46, 31, 9, 95, 43, 19, 47, 64, 48, 95, 80, 31, 47, 19,
      72, 99, 28, 46, 13, 9, 64, 4, 68, 74, 50, 28, 69, 94, 93, 3, 80, 78, 23, 80, 43, 49, 77, 18,
      68, 28, 13, 61, 34, 44, 80, 70, 55, 85, 0, 37, 93, 40, 47, 47, 45, 23, 26, 74, 45, 67, 34,
      20, 33, 71, 48, 96
    ])
  );

  println!(
    "{}",
    Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296])
  );
}
