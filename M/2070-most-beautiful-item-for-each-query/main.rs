struct Solution {}

impl Solution {
  pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut items = items;
    items.sort_by(|x, y| x[0].cmp(&y[0]).then_with(|| x[1].cmp(&y[1])));

    let mut buf: Vec<(i32, i32)> = vec![(0, 0)];
    let mut max: i32 = 0;
    items.iter().for_each(|v| {
      let s = buf.len();
      max = max.max(v[1]);
      if v[0] > buf[s - 1].0 {
        buf.push((v[0], max));
      } else {
        buf[s - 1].1 = max;
      }
    });

    let mut ans: Vec<i32> = vec![];
    queries.iter().for_each(|&q| {
      let offset = match buf.binary_search_by(|p| p.0.cmp(&q)) {
        Ok(ov) => ov,
        Err(ev) => ev,
      };
      if offset < buf.len() && buf[offset].0 == q {
        ans.push(buf[offset].1);
      } else {
        ans.push(buf[offset - 1].1);
      }
    });
    ans
  }
}
