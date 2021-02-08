struct Solution {}

impl Solution {
  pub fn knight_dialer(n: i32) -> i32 {
    let mut buf: Vec<i32> = vec![1; 10];

    for i in 1..n {
      let c0 = buf[0];
      let c1 = buf[1];
      let c2 = buf[2];
      let c3 = buf[3];
      let c4 = buf[4];
      let c5 = buf[5];
      let c6 = buf[6];
      let c7 = buf[7];
      let c8 = buf[8];
      let c9 = buf[9];
      for j in 0..10 {
        buf[j] = 0;
      }
      for j in 0..10 {
        if j == 0 {
          buf[4] += c0;
          buf[4] %= 1000000007;
          buf[6] += c0;
          buf[6] %= 1000000007;
        }
        if j == 1 {
          buf[6] += c1;
          buf[6] %= 1000000007;
          buf[8] += c1;
          buf[8] %= 1000000007;
        }
        if j == 2 {
          buf[7] += c2;
          buf[7] %= 1000000007;
          buf[9] += c2;
          buf[9] %= 1000000007;
        }
        if j == 3 {
          buf[4] += c3;
          buf[4] %= 1000000007;
          buf[8] += c3;
          buf[8] %= 1000000007;
        }
        if j == 4 {
          buf[3] += c4;
          buf[3] %= 1000000007;
          buf[9] += c4;
          buf[9] %= 1000000007;
          buf[0] += c4;
          buf[0] %= 1000000007;
        }
        if j == 6 {
          buf[1] += c6;
          buf[1] %= 1000000007;
          buf[7] += c6;
          buf[7] %= 1000000007;
          buf[0] += c6;
          buf[0] %= 1000000007;
        }
        if j == 7 {
          buf[2] += c7;
          buf[2] %= 1000000007;
          buf[6] += c7;
          buf[6] %= 1000000007;
        }
        if j == 8 {
          buf[1] += c8;
          buf[1] %= 1000000007;
          buf[3] += c8;
          buf[3] %= 1000000007;
        }
        if j == 9 {
          buf[4] += c9;
          buf[4] %= 1000000007;
          buf[2] += c9;
          buf[2] %= 1000000007;
        }
      }
    }

    let mut sum = 0;
    for i in 0..10 {
      sum += buf[i];
      sum %= 1000000007;
    }
    sum
  }
}

fn main() {
  println!("{}", Solution::knight_dialer(3));
}
