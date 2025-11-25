impl Solution {
  pub fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    let mut i: usize = 1;
    while i < weight.len() {
      if weight[i] < weight[i - 1] {
        ans += 1;
        i += 1;
      }
      i += 1;
    }
    ans
  }
}
