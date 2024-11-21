impl Solution {
  pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut dp: Vec<i32> = Vec::new();
    (0..n).for_each(|idx| {
      dp.push(idx);
    });

    let mut ans: Vec<i32> = Vec::new();
    queries.iter().for_each(|query| {
      let s = dp.binary_search(&query[0]);
      let e = dp.binary_search(&query[1]);
      if s.is_ok() && e.is_ok() {
        dp.drain(s.unwrap() + 1..e.unwrap());
      }
      // println!("{:?}", dp);
      ans.push(dp.len() as i32 - 1);
    });
    ans
  }
}
