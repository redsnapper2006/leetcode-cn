struct Solution {}

impl Solution {
  pub fn check_partitioning(s: String) -> bool {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    let bb = s.as_bytes().to_vec();
    let size = bb.len();

    fn is_pal(bb: &Vec<u8>, start: usize, end: usize) -> bool {
      let mut s = start as i32;
      let mut e = end as i32;
      while s <= e {
        if bb[s as usize] != bb[e as usize] {
          return false;
        }
        s += 1;
        e -= 1;
      }
      true
    }

    (0..bb.len() - 2).for_each(|idx| {
      if bb[idx] == bb[0] && is_pal(&bb, 0, idx) {
        left.push(idx);
      }
    });
    (2..bb.len()).for_each(|idx| {
      if bb[idx] == bb[size - 1] && is_pal(&bb, idx, size - 1) {
        right.push(idx);
      }
    });

    let mut index: usize = 0;
    while index < left.len() {
      let rr = match right.binary_search(&(left[index] + 2)) {
        Ok(ov) => ov,
        Err(ev) => ev,
      };
      if rr == right.len() {
        break;
      }

      if is_pal(&bb, left[index] + 1, right[rr] - 1) {
        return true;
      }

      index += 1;
    }
    false
  }
}

fn main() {
  println!("{}", Solution::check_partitioning("abcbdd".to_string()));
  println!("{}", Solution::check_partitioning("bcbddxy".to_string()));
}
