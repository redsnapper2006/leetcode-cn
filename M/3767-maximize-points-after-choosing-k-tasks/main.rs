impl Solution {
  pub fn max_points(technique1: Vec<i32>, technique2: Vec<i32>, k: i32) -> i64 {
    let mut buf: Vec<(i64, usize)> = vec![];
    for i in 0..technique1.len() {
      buf.push(((technique1[i] - technique2[i]) as i64, i));
    }
    buf.sort_unstable();

    let mut ans: i64 = 0;
    for i in 0..buf.len() {
      ans += if (i + k as usize) < buf.len() && buf[i].0 <= 0 {
        technique2[buf[i].1] as i64
      } else {
        technique1[buf[i].1] as i64
      };
    }
    ans
  }
}
