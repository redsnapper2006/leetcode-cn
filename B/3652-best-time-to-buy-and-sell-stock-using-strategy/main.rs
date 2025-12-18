impl Solution {
  pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut buf: Vec<i64> = vec![0; prices.len()];
    let mut w: i64 = 0;
    let mut mx: i64 = 0;
    for i in 0..prices.len() {
      buf[i] = if i > 0 { buf[i - 1] } else { 0 } + strategy[i] as i64 * prices[i] as i64;
      w += if i < k / 2 { 0 } else { 1 } * prices[i] as i64;
      if i >= k {
        w -= prices[i - k / 2] as i64;
      }

      if i >= k - 1 {
        mx = mx.max(w - buf[i] + if i >= k { buf[i - k] } else { 0 });
      }
    }
    buf[buf.len() - 1] + mx
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::max_profit(vec![4, 2, 8], vec![-1, 0, 1], 2));

  println!("{}", Solution::max_profit(vec![5, 4, 3], vec![1, 1, 0], 2));
}
