impl Solution {
  pub fn smallest_number(n: i32, t: i32) -> i32 {
    let digits_product = |mut x: i32| -> i32 {
      let mut product = 1i32;
      while x > 0 {
        product *= x % 10;
        x /= 10;
      }
      product
    };

    for i in 0..10 {
      if digits_product(n + i) % t == 0 {
        return n + i;
      }
    }

    unreachable!()
  }
}
