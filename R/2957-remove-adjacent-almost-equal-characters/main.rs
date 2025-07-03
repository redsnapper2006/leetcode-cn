impl Solution {
  pub fn remove_almost_equal_characters(word: String) -> i32 {
    let mut ans: i32 = 0;
    let bb = word.as_bytes().to_vec();
    let mut cnt: i32 = 0;
    for i in 1..bb.len() {
      if (bb[i] as i32 - bb[i - 1] as i32).abs() <= 1 {
        cnt += 1;
      } else {
        ans += (cnt + 1) / 2;
        cnt = 0;
      }
    }
    ans + (cnt + 1) / 2
  }
}
