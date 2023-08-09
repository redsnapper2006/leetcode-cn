impl Solution {
  pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut digits: Vec<i32> = Vec::new();
    let mut n = n;
    while n > 0 {
      digits.push(n % 10);
      n /= 10;
    }
    digits.clone().into_iter().product::<i32>() - digits.clone().into_iter().sum::<i32>()
  }
}
