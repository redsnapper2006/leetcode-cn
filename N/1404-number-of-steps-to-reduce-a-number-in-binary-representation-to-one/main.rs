impl Solution {
  pub fn num_steps(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut plus: bool = false;
    let mut ans: i32 = 0;
    for i in (1..bb.len()).rev() {
      let b = bb[i] - b'0' + if plus { 1 } else { 0 };

      ans += 2 - (b as i32 + 1) % 2;
      plus = b >= 1;
    }
    ans + if plus { 1 } else { 0 }
  }
}
