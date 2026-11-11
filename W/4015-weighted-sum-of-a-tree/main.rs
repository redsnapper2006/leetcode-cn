use std::collections::HashMap;

impl Solution {
  pub fn weighted_sum(parent: Vec<i32>, nums: Vec<i32>) -> i64 {
    let mut p: HashMap<i32, Vec<i32>> = HashMap::new();
    for (i, &n) in parent.iter().enumerate() {
      p.entry(n).or_insert(vec![]).push(i as i32);
    }

    let mut mx_height = 0i64;
    fn height(cur: i32, p: &HashMap<i32, Vec<i32>>, h: i64, mx: &mut i64) {
      if let None = p.get(&cur) {
        (*mx) = (*mx).max(h);
        return;
      }
      for &n in p.get(&cur).unwrap() {
        height(n, p, h + 1, mx);
      }
    };
    height(-1, &p, 0, &mut mx_height);

    fn dfs(cur: i32, p: &HashMap<i32, Vec<i32>>, nums: &[i32], h: i64, mx: i64, ans: &mut i64) {
      *ans += nums[cur as usize] as i64 * (mx - h + 1);
      if !p.contains_key(&cur) {
        return;
      }
      for &n in p.get(&cur).unwrap() {
        dfs(n, p, nums, h + 1, mx, ans);
      }
    }

    let mut ans = 0i64;
    dfs(0, &p, &nums, 1, mx_height, &mut ans);
    ans
  }
}
