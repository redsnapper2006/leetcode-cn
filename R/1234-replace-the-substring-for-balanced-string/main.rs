struct Solution {}

impl Solution {
  pub fn balanced_string(s: String) -> i32 {
    let mut count: [i32; 4] = [0; 4];
    for b in s.as_bytes() {
      if *b == 'Q' as u8 {
        count[0] += 1;
      }
      if *b == 'W' as u8 {
        count[1] += 1;
      }
      if *b == 'E' as u8 {
        count[2] += 1;
      }
      if *b == 'R' as u8 {
        count[3] += 1;
      }
    }

    let mut diff: [i32; 4] = [0; 4];
    (0..4).for_each(|idx| {
      if count[idx] > s.len() as i32 / 4 {
        diff[idx] = count[idx] - s.len() as i32 / 4;
      } else {
        diff[idx] = 0;
      }
    });
    if (0..4).map(|idx| diff[idx] == 0).all(|x| x) {
      return 0;
    }

    fn check(cur: [i32; 4], diff: [i32; 4]) -> bool {
      (0..4)
        .map(|idx| match (diff[idx], cur[idx]) {
          (0, _) => true,
          (d, c) => c >= d,
        })
        .all(|x| x)
    }

    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut cur: [i32; 4] = [0; 4];
    let mut ret: i32 = s.len() as i32;
    while end < s.len() {
      while !check(cur, diff) && end < s.len() {
        let b = s.as_bytes()[end];
        if b == 'Q' as u8 {
          cur[0] += 1;
        }
        if b == 'W' as u8 {
          cur[1] += 1;
        }
        if b == 'E' as u8 {
          cur[2] += 1;
        }
        if b == 'R' as u8 {
          cur[3] += 1;
        }
        end += 1;
      }
      while start <= end && check(cur, diff) {
        if ret > (end - start) as i32 {
          ret = (end - start) as i32;
        }
        let b = s.as_bytes()[start];
        if b == 'Q' as u8 {
          cur[0] -= 1;
        }
        if b == 'W' as u8 {
          cur[1] -= 1;
        }
        if b == 'E' as u8 {
          cur[2] -= 1;
        }
        if b == 'R' as u8 {
          cur[3] -= 1;
        }
        start += 1;
      }
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::balanced_string("QQWE".to_string()));
}
