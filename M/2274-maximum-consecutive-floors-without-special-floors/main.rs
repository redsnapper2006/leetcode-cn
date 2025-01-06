struct Solution {}

impl Solution {
  pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
    let mut ss = special;
    ss.push(bottom - 1);
    ss.push(top + 1);
    ss.sort_unstable();
    let mut ans: i32 = 0;
    (1..ss.len()).for_each(|idx| {
      ans = ans.max(ss[idx] - ss[idx - 1] - 1);
    });
    ans
  }
}
