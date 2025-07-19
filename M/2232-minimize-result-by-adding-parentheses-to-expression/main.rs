impl Solution {
  pub fn minimize_result(expression: String) -> String {
    let bb = expression.as_bytes().to_vec();
    let mut d1: Vec<i32> = vec![];
    let mut d2: Vec<i32> = vec![];
    let mut first: bool = true;
    for i in 0..bb.len() {
      if bb[i] == b'+' {
        first = false;
        continue;
      }
      let d = (bb[i] - b'0') as i32;
      if first {
        d1.push(d);
      } else {
        d2.push(d);
      }
    }

    let mut mnx: i32 = i32::MAX;
    let mut ans: Vec<u8> = vec![];
    for i in 0..d1.len() {
      let mut ll: i32 = 0;
      for ii in 0..i {
        ll = ll * 10 + d1[ii];
      }
      if ll == 0 {
        ll = 1;
      }
      let mut p1: i32 = 0;
      for ii in i..d1.len() {
        p1 = p1 * 10 + d1[ii];
      }

      for j in 1..d2.len() + 1 {
        let mut p2: i32 = 0;
        for ii in 0..j {
          p2 = p2 * 10 + d2[ii];
        }
        let mut rr: i32 = 0;
        for ii in j..d2.len() {
          rr = rr * 10 + d2[ii];
        }
        if rr == 0 {
          rr = 1;
        }

        if ll * (p1 + p2) * rr < mnx {
          mnx = ll * (p1 + p2) * rr;
          ans = vec![];
          for ii in 0..i {
            ans.push((d1[ii] as u8 + b'0'));
          }
          ans.push(b'(');
          for ii in i..d1.len() {
            ans.push((d1[ii] as u8 + b'0'));
          }
          ans.push(b'+');
          for ii in 0..j {
            ans.push((d2[ii] as u8 + b'0'));
          }
          ans.push(b')');
          for ii in j..d2.len() {
            ans.push((d2[ii] as u8 + b'0'));
          }
        }
      }
    }
    String::from_utf8(ans).unwrap()
  }
}
