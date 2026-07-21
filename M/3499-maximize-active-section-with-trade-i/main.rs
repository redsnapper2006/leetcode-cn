impl Solution {
  pub fn max_active_sections_after_trade(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut ones: i32 = 0;
    let mut zeros: Vec<i32> = vec![];
    let mut zero: i32 = 0;
    for i in 0..bb.len() {
      if bb[i] == b'1' {
        ones += 1;
        if zero > 0 {
          zeros.push(zero);
        }
        zero = 0;
      } else {
        zero += 1;
      }
    }
    if zero > 0 {
      zeros.push(zero);
    }

    let mut zero_group: i32 = 0;
    if zeros.len() >= 2 {
      for i in 0..zeros.len() - 1 {
        zero_group = zero_group.max(zeros[i] + zeros[i + 1]);
      }
    }

    ones + zero_group
  }
}
