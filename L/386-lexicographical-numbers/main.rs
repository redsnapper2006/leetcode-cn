impl Solution {
  pub fn lexical_order(n: i32) -> Vec<i32> {
    fn dfs(b: i32, n: i32, ans: &mut Vec<i32>) {
      if b > n {
        return;
      }
      ans.push(b);
      (0..10).for_each(|idx| {
        dfs(b * 10 + idx, n, ans);
      });
    }
    let mut ans: Vec<i32> = vec![];
    (1..10).for_each(|idx| {
      dfs(idx, n, &mut ans);
    });
    ans
  }
}

struct Solution {}
fn main() {
  println!("{:?}", Solution::lexical_order(10));
}
