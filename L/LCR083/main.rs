use std::collections::HashSet;
impl Solution {
  pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(nums: &Vec<i32>, t: &mut Vec<i32>, visit: &mut HashSet<i32>, ans: &mut Vec<Vec<i32>>) {
      if t.len() == nums.len() {
        ans.push(t.clone());
        return;
      }
      // println!("{:?}", t);
      (0..nums.len()).for_each(|idx| {
        if visit.contains(&nums[idx]) {
          return;
        }
        visit.insert(nums[idx]);
        t.push(nums[idx]);
        dfs(nums, t, visit, ans);
        visit.remove(&nums[idx]);
        t.pop();
      });
    }

    let mut t: Vec<i32> = vec![];
    let mut visit: HashSet<i32> = HashSet::new();
    let mut ans: Vec<Vec<i32>> = vec![];

    dfs(&nums, &mut t, &mut visit, &mut ans);
    ans
  }
}

struct Solution{}
fn main() {
  println!("{:?}", Solution::permute(vec![1,2,3]));
}
