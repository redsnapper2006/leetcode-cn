use std::collections::BTreeSet;
impl Solution {
  pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
    let mut h_fences = h_fences;
    h_fences.push(1);
    h_fences.push(m);
    h_fences.sort_unstable();
    let mut h_diff: BTreeSet<i64> = BTreeSet::new();
    for i in 0..h_fences.len() {
      for j in i + 1..h_fences.len() {
        h_diff.insert((h_fences[j] - h_fences[i]) as i64);
      }
    }

    let mut v_fences = v_fences;
    v_fences.push(1);
    v_fences.push(n);
    v_fences.sort_unstable();
    let v_max: i64 = 0;
    for i in 0..v_fences.len() {
      for j in i + 1..v_fences.len() {
        let v = (v_fences[j] - v_fences[i]) as i64;
        if h_diff.contains(&v) {
          v_max = v_max.max(v);
        }
      }
    }
    if v_max == 0 {
      -1
    } else {
      ((v_max * v_max) % 1000000007) as _
    }
  }
}
