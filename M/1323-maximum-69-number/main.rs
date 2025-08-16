impl Solution {
  pub fn maximum69_number(num: i32) -> i32 {
    let mut buf: Vec<i32> = vec![];
    let mut n = num;
    while n > 0 {
      buf.push(n % 10);
      n /= 10;
    }

    for i in (0..buf.len()).rev() {
      if buf[i] == 6 {
        buf[i] = 9;
        break;
      }
    }

    let mut ans: i32 = 0;
    for i in (0..buf.len()).rev() {
      ans = ans * 10 + buf[i];
    }

    ans
  }
}
