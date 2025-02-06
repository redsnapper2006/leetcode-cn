struct Solution {}

impl Solution {
  pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ans: Vec<Vec<i32>> = vec![];
    let mut visit: Vec<bool> = vec![false; nums.len()];
    let mut cur: Vec<i32> = vec![];
    fn dfs(nums: &Vec<i32>, cur: &mut Vec<i32>, visit: &mut Vec<bool>, ans: &mut Vec<Vec<i32>>) {
      if cur.len() == nums.len() {
        ans.push(cur.clone());
        return;
      }

      (0..nums.len()).for_each(|idx| {
        if visit[idx] || (idx > 0 && nums[idx] == nums[idx - 1] && !visit[idx - 1]) {
          return;
        }
        cur.push(nums[idx]);
        visit[idx] = true;
        dfs(nums, cur, visit, ans);
        visit[idx] = false;
        cur.pop();
      });
    }
    dfs(&nums, &mut cur, &mut visit, &mut ans);

    ans
  }
}

fn main() {
  println!("{:?}", Solution::permute_unique(vec![1, 1, 2]));
  println!("{:?}", Solution::permute_unique(vec![1, 2, 3]));
}
