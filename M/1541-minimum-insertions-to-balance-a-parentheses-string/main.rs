struct Solution {}

impl Solution {
  pub fn min_insertions(s: String) -> i32 {
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

fn main() {
  println!("{}", Solution::min_insertions("(()))".to_string()));

  println!("{}", Solution::min_insertions("()()()))".to_string()));
}
