struct Solution {}

impl Solution {
  pub fn min_operations(nums: Vec<i32>) -> i32 {
    let size = nums.len() as i32;

    let mut nums = nums;
    nums.sort();
    nums.dedup();

    let mut min = nums.len() as i32 + 1;
    (0..nums.len()).for_each(|idx| {
      let target = nums[idx] - size + 1;

      let offset = match nums.binary_search(&target) {
        Ok(off) => off,
        Err(off) => off,
      };
      min = min.min(size - (idx - offset) as i32 - 1);
    });

    // (0..nums.len()).for_each(|idx| {
    //   let target = nums[idx] + size - 1;

    //   let offset = match nums.binary_search(&target) {
    //     Ok(off) => off,
    //     Err(off) => off,
    //   };

    //   let mut cnt = offset - idx;
    //   if offset < nums.len() && nums[offset] == target {
    //     cnt += 1;
    //   }
    //   min = min.min(size - cnt as i32);
    // });
    min
  }
}

fn main() {
  println!("{}", Solution::min_operations(vec![4, 2, 5, 3]));
  println!("{}", Solution::min_operations(vec![1, 2, 3, 5, 6]));
  println!("{}", Solution::min_operations(vec![1, 10, 100, 1000]));
  println!(
    "{}",
    Solution::min_operations(vec![10, 49, 16, 22, 28, 34, 35, 7])
  );

  println!("{}", Solution::min_operations(vec![4, 5, 8, 8, 9, 9]));
}
