impl Solution {
  pub fn find_max_val(n: i32, restrictions: Vec<Vec<i32>>, diff: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut buf: Vec<i32> = vec![i32::MAX; n];
    for res in &restrictions {
      buf[res[0] as usize] = res[1];
    }
    buf[0] = 0;

    for i in 1..n {
      buf[i] = buf[i].min(buf[i - 1] + diff[i - 1]);
    }
    for i in (0..n - 1).rev() {
      buf[i] = buf[i].min(buf[i + 1] + diff[i]);
    }

    *buf.iter().max().unwrap()
  }
}
