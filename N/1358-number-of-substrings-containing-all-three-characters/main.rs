impl Solution {
  pub fn number_of_substrings2(s: String) -> i32 {
    let mut ans: usize = 0;
    let (mut start, mut end): (usize, usize) = (0, 0);
    let (mut a, mut b, mut c): (i32, i32, i32) = (0, 0, 0);
    fn sum(a: &mut i32, b: &mut i32, c: &mut i32, bb: u8, step: i32) {
      match bb {
        b'a' => *a += step,
        b'b' => *b += step,
        _ => *c += step,
      };
    }
    let bb = s.as_bytes();
    while end < bb.len() {
      sum(&mut a, &mut b, &mut c, bb[end], 1);
      while a >= 1 && b >= 1 && c >= 1 {
        sum(&mut a, &mut b, &mut c, bb[start], -1);
        start += 1;
      }
      ans += start;
      end += 1;
    }
    ans as _
  }

  pub fn number_of_substrings(s: String) -> i32 {
    let mut idx: Vec<i32> = vec![-1; 3];
    let mut res: i32 = 0;
    s.as_bytes().iter().enumerate().for_each(|(ii, &bb)| {
      idx[(match bb {
        b'a' => 0,
        b'b' => 1,
        _ => 2,
      })] = ii as i32;
      res += idx[0].min(idx[1]).min(idx[2]) + 1;
    });
    res
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::number_of_substrings("abcabc".to_string()));
  println!("{}", Solution::number_of_substrings("aaacb".to_string()));
}
