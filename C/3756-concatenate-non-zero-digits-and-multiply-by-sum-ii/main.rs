impl Solution {
  pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    const MOD: i64 = 1000000007;
    let bb = s.as_bytes();
    let n = bb.len();

    let mut prefix_sum: Vec<i64> = vec![0; n + 1];
    let mut non_zero_count: Vec<usize> = vec![0; n + 1];
    let mut prefix_product: Vec<i64> = vec![0; n + 1];
    let mut power: Vec<i64> = vec![1; n + 1];

    for i in 1..=n {
      power[i] = (power[i - 1] * 10) % MOD;

      let digit = (bb[i - 1] - b'0') as i64;
      prefix_sum[i] = prefix_sum[i - 1] + digit;
      non_zero_count[i] = non_zero_count[i - 1];
      prefix_product[i] = prefix_product[i - 1];
      if digit > 0 {
        non_zero_count[i] = non_zero_count[i - 1] + 1;
        prefix_product[i] = (prefix_product[i - 1] * 10 + digit) % MOD;
      }
    }

    queries
      .iter()
      .map(|q| {
        let l = q[0] as usize;
        let r = q[1] as usize + 1;
        let product_diff = (prefix_product[r]
          - (prefix_product[l] * power[non_zero_count[r] - non_zero_count[l]]) % MOD
          + MOD)
          % MOD;
        (product_diff * (prefix_sum[r] - prefix_sum[l]) % MOD) as i32
      })
      .collect()
  }
}
