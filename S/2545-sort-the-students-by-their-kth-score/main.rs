struct Solutioin {}

impl Solution {
  pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut sc = score;
    sc.sort_by_key(|s| -s[k as usize]);
    sc
  }
}
