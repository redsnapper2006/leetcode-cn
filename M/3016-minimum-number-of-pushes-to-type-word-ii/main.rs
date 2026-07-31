impl Solution {
  pub fn minimum_pushes(word: String) -> i32 {
    let mut cnt = [0i32; 26];
    for b in word.bytes() {
      cnt[(b - b'a') as usize] += 1;
    }
    cnt.sort_unstable_by(|a, b| b.cmp(a));

    let mut ans = 0i32;
    for idx in 0..26i32 {
      ans += cnt[idx as usize] * ((idx >> 3) + 1);
    }
    ans
  }
}
