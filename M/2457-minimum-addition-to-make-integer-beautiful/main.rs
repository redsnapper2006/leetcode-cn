impl Solution {
  pub fn make_integer_beautiful(n: i64, target: i32) -> i64 {
    let mut buf: Vec<i64> = vec![0; 64];
    let mut off: usize = 0;
    let mut nn = n;
    let target = target as i64;
    let mut sum: i64 = 0;
    while nn > 0 {
      let m = nn % 10;
      buf[off] = m;
      sum += m;
      off += 1;
      nn /= 10;
    }
    let mut off: usize = 0;
    while sum > target {
      while buf[off + 1] == 9 {
        sum -= buf[off];
        buf[off] = 0;
        off += 1;
      }
      sum -= buf[off];
      buf[off] = 0;
      buf[off + 1] += 1;
      sum += 1;
      off += 1;
    }

    let mut ans: i64 = 0;
    for i in (0..64).rev() {
      ans = ans * 10 + buf[i];
    }
    ans - n
  }
}
