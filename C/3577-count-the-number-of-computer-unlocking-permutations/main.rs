impl Solution {
  pub fn count_permutations(complexity: Vec<i32>) -> i32 {
    let mut ans: i64 = 1;
    for i in 1..complexity.len() {
      if complexity[i] <= complexity[0] {
        return 0;
      }
      ans = ans * i as i64 % 1000000007;
    }
    ans
  }
}
