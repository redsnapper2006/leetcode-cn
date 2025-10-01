impl Solution {
  pub fn get_least_frequent_digit(n: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; 10];
    let mut n = n;
    while n > 0 {
      let m = n % 10;
      buf[m as usize] += 1;
      n /= 10;
    }
    let mut ans: i32 = -1;
    let mut ans_mn: i32 = 100;
    for i in 0..10 {
      if buf[i] == 0 {
        continue;
      }
      if ans_mn > buf[i] {
        ans_mn = buf[i];
        ans = i as i32;
      }
    }
    ans
  }
}
