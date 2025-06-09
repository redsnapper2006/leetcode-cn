impl Solution {
  pub fn max_product(n: i32) -> i32 {
    let mut dp : Vec<i32> = vec![0;10];
    let mut n = n as usize;
    while n > 0 {
      dp[n %10 ] +=1;
      n /=10;
    }
    let mut m1 : i32 = -1;
    let mut m2 : i32 = -1;

    let mut idx : usize = 9;
    while idx > 0 &&( m1 == -1 || m2 == -1 ) {
      if dp[idx] == 0 {
        idx -=1;
        continue;
      }

      if m1 == -1 {
        m1 = idx as i32;
        dp[idx] -= 1;
      } else {
        return m1 * (idx as i32);
      }
    }
    0
  }
}
