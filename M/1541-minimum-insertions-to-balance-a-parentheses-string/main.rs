impl Solution {
  pub fn min_insertions(s: String) -> i32 {
    let (mut need, mut ans) = (0i32, 0i32);
    for b in s.bytes() {
      if b == b')' {
        need -= 1;
        if need < 0 {
          ans += 1;
          need += 2;
        }
      } else {
        need += 2;
        if need & 1 == 1 {
          ans += 1;
          need -= 1;
        }
      }
    }

    ans + need
  }

  pub fn min_insertions2(s: String) -> i32 {
    let mut cnt: i32 = 0;
    let bb: Vec<u8> = s.as_bytes().to_vec();

    let mut right: Vec<usize> = Vec::new();
    let mut right_cnt: i32 = 0;
    (0..bb.len()).rev().for_each(|idx| {
      if bb[idx] == '(' as u8 {
        if right_cnt > 0 {
          right_cnt -= 1;
        } else {
          if right.len() > 0 {
            right.pop();
            cnt += 1;
          } else {
            cnt += 2;
          }
        }
      } else {
        if right.len() > 0 && right[right.len() - 1] - 1 == idx {
          right_cnt += 1;
          right.pop();
        } else {
          right.push(idx);
        }
      }
    });
    cnt + right_cnt + right.len() as i32 * 2
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::min_insertions("(()))".to_string()));

  println!("{}", Solution::min_insertions("()()()))".to_string()));
}
