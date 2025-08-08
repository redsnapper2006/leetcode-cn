use std::collections::HashSet;
impl Solution {
  pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
    fn get_diff(fences: &Vec<i32>) -> Vec<i64> {
      let mut hs: HashSet<i64> = HashSet::new();
      for i in 0..fences.len() {
        hs.insert((fences[i] - 1) as i64);
        for ii in 0..i {
          hs.insert((fences[i] - fences[ii]) as i64);
        }
      }
      let mut diff: Vec<i64> = vec![];
      for k in hs.iter() {
        diff.push(*k);
      }
      diff.sort_unstable();
      diff
    }
    let mut h_fences = h_fences;
    h_fences.push(m);
    h_fences.sort_unstable();
    let h_diff = get_diff(&h_fences);
    let mut v_fences = v_fences;
    v_fences.push(n);
    v_fences.sort_unstable();
    let v_diff = get_diff(&v_fences);

    let mut ans: i32 = -1;
    let mut i1: i32 = h_diff.len() as i32 - 1;
    let mut i2: i32 = v_diff.len() as i32 - 1;
    while i1 >= 0 && i2 >= 0 {
      if h_diff[i1 as usize] == v_diff[i2 as usize] {
        return ((h_diff[i1 as usize] * h_diff[i1 as usize]) % 1000000007) as i32;
      } else if h_diff[i1 as usize] > v_diff[i2 as usize] {
        i1 -= 1;
      } else {
        i2 -= 1;
      }
    }
    -1
  }
}
