impl Solution {
  pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
    fn is_prime(n: i32) -> bool {
      n == 2
        || n > 1
          && n % 2 > 0
          && (3..=(n as f64).sqrt() as i32).step_by(2).all(|i| n % i > 0)
    }

    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 2..=(n / 2) {
      if is_prime(i) && is_prime(n - i) {
        ans.push(vec![i, n - i]);
      }
    }
    ans
  }
}
