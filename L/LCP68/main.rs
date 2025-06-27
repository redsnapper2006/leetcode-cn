impl Solution {
  pub fn beautiful_bouquet(flowers: Vec<i32>, cnt: i32) -> i32 {
    let cnt = cnt as i64;
    let mut start: usize = 0;
    let mut buf: Vec<i64> = vec![0; 100001];

    let mut ans: i64 = 0;
    for i in 0..flowers.len() {
      let off = flowers[i] as usize;
      buf[off] += 1;
      if buf[off] > cnt {
        while buf[off] > cnt {
          let o2 = flowers[start] as usize;
          buf[o2] -= 1;
          start += 1;
        }
      }
      ans += (i - start) as i64 + 1;
      ans %= 1000000007;
    }
    (ans % 1000000007) as _
  }
}
