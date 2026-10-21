impl Solution {
  pub fn min_operations(s: String) -> i32 {
    let bb = s.as_bytes();
    let n = bb.len();
    let (mut mn, mut mn_cnt, mut mx, mut mx_cnt) = (bb[0], 1i32, bb[0], 1i32);
    let mut sorted = true;
    for i in 1..bb.len() {
      if bb[i] < bb[i - 1] {
        sorted = false;
      }
      if mx < bb[i] {
        mx = bb[i];
        mx_cnt = 1;
      } else if mx == bb[i] {
        mx_cnt += 1;
      }
      if mn > bb[i] {
        mn = bb[i];
        mn_cnt = 1;
      } else if mn == bb[i] {
        mn_cnt += 1;
      }
    }

    if sorted {
      0
    } else if bb.len() == 2 {
      -1
    } else if bb[0] == mn || bb[n - 1] == mx {
      1
    } else {
      let mut ans = 2i32;
      if bb[0] == mx && bb[n - 1] == mn && mn_cnt == 1 && mx_cnt == 1 {
        ans += 1;
      }
      ans
    }
  }
}
