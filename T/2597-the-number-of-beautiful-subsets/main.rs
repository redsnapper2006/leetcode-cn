struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
    let mut set: HashMap<i32, i32> = HashMap::new();
    fn dfs(nums: &Vec<i32>, k: i32, index: usize, set: &mut HashMap<i32, i32>, ans: &mut i32) {
      if index == nums.len() {
        return;
      }
      set.entry(nums[index]).and_modify(|x| *x += 1).or_insert(1);
      *ans += 1;

      let mut idx: usize = index + 1;
      while idx < nums.len() {
        if !set.contains_key(&(nums[idx] - k)) && !set.contains_key(&(nums[idx] + k)) {
          dfs(nums, k, idx, set, ans);
        }
        idx += 1;
      }
      set.entry(nums[index]).and_modify(|x| *x -= 1);
      if *set.get(&nums[index]).unwrap() == 0 {
        set.remove(&nums[index]);
      }
    }
    let mut ans: i32 = 0;
    (0..nums.len()).for_each(|index| {
      dfs(&nums, k, index, &mut set, &mut ans);
    });
    ans
  }
}

fn main() {
  println!("{}", Solution::beautiful_subsets(vec![2, 4, 6], 2));
  println!("{}", Solution::beautiful_subsets(vec![1], 1));
  println!(
    "{}",
    Solution::beautiful_subsets(vec![1, 2, 4, 5, 7, 10], 3)
  );
  println!("{}", Solution::beautiful_subsets(vec![1, 1, 2], 1));
}
