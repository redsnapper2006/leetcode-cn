impl Solution {
  pub fn minimum_cost(s: String) -> i64 {
    let bb = s.as_bytes().to_vec();
    let mut ans: i64 = 0;
    (1..bb.len()).for_each(|idx| {
      if bb[idx] != bb[idx - 1] {
        ans += idx.min(bb.len() - idx) as i64;
      }
    });
    ans
  }

  pub fn minimum_cost2(s: String) -> i64 {
    let bb = s.as_bytes().to_vec();
    let mut left_zero: Vec<i64> = vec![0; bb.len()];
    let mut right_zero: Vec<i64> = vec![0; bb.len()];
    let mut left_one: Vec<i64> = vec![0; bb.len()];
    let mut right_one: Vec<i64> = vec![0; bb.len()];

    (0..bb.len()).for_each(|idx| {
      let idx64 = idx as i64;
      if bb[idx] == b'0' {
        if idx == 0 {
          left_one[idx] = 1;
        } else {
          if bb[idx - 1] == b'0' {
            left_one[idx] = left_one[idx - 1] + 1;
          } else {
            left_one[idx] = left_one[idx - 1] + idx64 * 2 + 1;
          }
        }
        if idx > 0 {
          left_zero[idx] = left_zero[idx - 1];
        }
      } else {
        if idx == 0 {
          left_zero[idx] = 1;
        } else {
          if bb[idx - 1] == b'1' {
            left_zero[idx] = left_zero[idx - 1] + 1;
          } else {
            left_zero[idx] = left_zero[idx - 1] + idx64 * 2 + 1;
          }
        }
        if idx > 0 {
          left_one[idx] = left_one[idx - 1];
        }
      }
    });

    (0..bb.len()).rev().for_each(|idx| {
      let idx64 = idx as i64;
      if bb[idx] == b'0' {
        if idx == bb.len() - 1 {
          right_one[idx] = 1;
        } else {
          if bb[idx + 1] == b'0' {
            right_one[idx] = right_one[idx + 1] + 1;
          } else {
            right_one[idx] = right_one[idx + 1] + (bb.len() as i64 - 1 - idx64) * 2 + 1
          }
        }
        if idx < bb.len() - 1 {
          right_zero[idx] = right_zero[idx + 1];
        }
      } else {
        if idx == bb.len() - 1 {
          right_zero[idx] = 1;
        } else {
          if bb[idx + 1] == b'1' {
            right_zero[idx] = right_zero[idx + 1] + 1;
          } else {
            right_zero[idx] = right_zero[idx + 1] + (bb.len() as i64 - 1 - idx64) * 2 + 1
          }
        }
        if idx < bb.len() - 1 {
          right_one[idx] = right_one[idx + 1];
        }
      }
    });

    // println!("zero {:?} {:?}", left_zero, right_zero);

    // println!("one {:?} {:?}", left_one, right_one);
    let mut ans: i64 = (bb.len() as i64 * 2 + 1) * bb.len() as i64 / 2;
    (0..bb.len()).for_each(|idx| {
      let mut zero: i64 = left_zero[idx];
      if idx < bb.len() - 1 {
        zero += right_zero[idx + 1];
      }
      let mut one: i64 = left_one[idx];
      if idx < bb.len() - 1 {
        one += right_one[idx + 1];
      }
      ans = ans.min(zero).min(one);
      // println!("ans {} {}", idx, ans);
    });
    ans
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::minimum_cost("0011".to_string()));
  println!("{}", Solution::minimum_cost("010101".to_string()));
  println!("{}", Solution::minimum_cost("010101010".to_string()));
}
