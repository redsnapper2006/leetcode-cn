struct Solution {}

impl Solution {
  pub fn add_minimum(word: String) -> i32 {
    let mut res: i32 = 0;
    let mut stack: u8 = b' ';
    let mut step: i32 = -1;
    word.as_bytes().iter().for_each(|&c| {
      if c == b'a' {
        if stack != b' ' {
          if stack == b'a' {
            res += 2;
            return;
          } else if stack == b'b' {
            res += 1;
          }
        }
        stack = c;
        return;
      }
      if c == b'b' {
        if stack == b' ' {
          res += 1;
        }
        if stack == b'a' {}
        if stack == b'b' {
          res += 2;
        }
        stack = c;
        return;
      }
      if c == b'c' {
        if stack == b' ' {
          res += 2;
        }
        if stack == b'a' {
          res += 1;
        }
        if stack == b'b' {}

        stack = b' '
      }
    });

    if stack == b'a' {
      res += 2;
    } else if stack == b'b' {
      res += 1;
    }

    res
  }
}
