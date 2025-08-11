impl Solution {
  pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut buf: Vec<i64> = vec![];
    let mut n = n;
    for i in 0..31 {
      if n & (1 << i) != 0 {
        buf.push((1 << i) as i64);
      }
    }

    queries
      .iter()
      .map(|query| {
        let s = query[0] as usize;
        let e = query[1] as usize;
        let mut product: i64 = 1;
        for j in s..=e {
          product *= buf[j];
          product %= 1000000007;
        }
        product as i32
      })
      .collect::<Vec<i32>>()
  }
}
