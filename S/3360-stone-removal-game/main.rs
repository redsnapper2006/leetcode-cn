impl Solution {
  pub fn can_alice_win(n: i32) -> bool {
    let mut base: i32 = 10;
    let mut times: i32 = 0;
    let mut n = n;
    while n > 0 && base > 0 && n >= base {
      n -= base;
      base -= 1;
      times += 1;
    }
    times % 2 == 1
  }
}
