impl Solution {
  pub fn max_weight(pizzas: Vec<i32>) -> i64 {
    let mut pizzas = pizzas;
    pizzas.sort_unstable_by(|a, b| b.cmp(a));
    let n = pizzas.len() as i32 / 4;
    let odd = (n + 1) / 2;
    let even = n - odd;
    let mut ans = 0i64;
    let mut idx = 0;
    (0..odd).for_each(|_| {
      ans += pizzas[idx] as i64;
      idx += 1;
    });
    (0..even).for_each(|_| {
      idx += 1;
      ans += pizzas[idx] as i64;
      idx += 1;
    });
    ans
  }
}
