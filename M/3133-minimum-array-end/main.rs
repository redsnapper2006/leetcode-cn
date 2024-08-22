struct Solution {}

impl Solution {
  pub fn min_end(n: i32, x: i32) -> i64 {
    let mut x = x as i64;

    let mut n = n - 1;
    let mut offset: i32 = 0;
    while n > 0 {
      if x & 1i64 << offset == 0 {
        if n & 1 == 1 {
          x |= 1i64 << offset;
        }
        n /= 2;
      }
      offset += 1;
    }
    x
  }

  pub fn min_end2(n: i32, x: i32) -> i64 {
    let mut x = x as i64;
    let mut buf: Vec<i32> = Vec::new();
    (0..64).for_each(|offset| {
      if x & 1i64 << offset == 0 {
        buf.push(offset);
      }
    });

    let mut n = n - 1;
    let mut offset: usize = 0;

    while n > 0 {
      if n & 1 == 1 {
        x |= 1 << buf[offset];
      }
      offset += 1;
      n /= 2;
    }
    x
  }
}

fn main() {
  println!("{}", Solution::min_end(6715154, 7193485));
}
