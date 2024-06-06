impl Solution {
  pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    a.into_iter()
      .zip(b.into_iter())
      .collect::<Vec<(i32, i32)>>()
      .iter()
      .fold(
        (Vec::new(), (0 as i64, 0 as i64)),
        |(mut ans, (sum_a, sum_b)), &(aa, bb)| {
          let aggr1 = sum_a + (1 << aa);
          let aggr2 = sum_b + (1 << bb);
          ans.push((aggr1 & aggr2).count_ones() as i32);
          (ans, (aggr1, aggr2))
        },
      )
      .0
  }
}
