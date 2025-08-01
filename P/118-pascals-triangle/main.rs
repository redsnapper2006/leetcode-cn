impl Solution {
  pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let nr = num_rows as usize;
    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 0..nr {
      ans.push(vec![1; i + 1]);
    }
    for i in 2..nr {
      for j in 1..i {
        ans[i][j] = ans[i - 1][j - 1] + ans[i - 1][j];
      }
    }
    ans
  }
}
