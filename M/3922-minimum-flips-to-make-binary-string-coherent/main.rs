impl Solution {
  pub fn min_flips(s: String) -> i32 {
    let bb = s.as_bytes();
    let mut ones = bb.iter().fold(0i32, |ans, &b| ans + (b as i32 & 1));
    let zeros = bb.len() as i32 - ones;
    ones -= 1;
    if bb[0] == b'1' && bb[s.len() - 1] == b'1' {
      ones -= 1;
    }
    zeros.min(ones.max(0))
  }

  pub fn min_flips2(s: String) -> i32 {
    let bb = s.as_bytes();
    let mut left: Vec<i32> = vec![0; bb.len()];
    for i in 0..bb.len() {
      left[i] = (bb[i] - b'0') as i32 + if i > 0 { left[i - 1] } else { 0 };
    }
    let mut right: Vec<i32> = vec![0; bb.len()];
    for i in (0..bb.len()).rev() {
      right[i] = (bb[i] - b'0') as i32 + if i < bb.len() - 1 { right[i + 1] } else { 0 };
    }

    let zeros = bb.iter().fold(0i32, |ans, &b| ans + if b & 1 == 0 { 1 } else { 0 });
    let mut ans: i32 = 0;

    for i in 0..bb.len() {
      if bb[i] == b'0' {
        ans = ans.max(
          if left[i] > 1 { left[i] - 1 } else { 0 } + if right[i] > 1 { right[i] - 1 } else { 0 },
        );
      }
    }
    ans.min(zeros)
  }
}
