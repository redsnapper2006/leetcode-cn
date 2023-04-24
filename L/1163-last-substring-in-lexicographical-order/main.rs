struct Solution {}

impl Solution {
  pub fn last_substring(s: String) -> String {
    let mut bb: Vec<u8> = s.as_bytes().to_vec();

    let mut i: usize = 0;
    let mut j: usize = 1;
    let mut k: usize = 0;

    while j + k < bb.len() {
      if bb[i + k] == bb[j + k] {
        k += 1;
      } else if bb[i + k] < bb[j + k] {
        i += k + 1;
        k = 0;
        if i >= j {
          j = i + 1;
        }
      } else {
        j += k + 1;
        k = 0;
      }
    }

    String::from_utf8(bb.drain(i..).collect()).unwrap()
  }

  pub fn last_substring2(s: String) -> String {
    let mut idxs: Vec<Vec<usize>> = vec![Vec::new(); 26];
    let mut max_offset: usize = 0;
    let mut bb: Vec<u8> = s.as_bytes().to_vec();

    bb.iter().enumerate().for_each(|(idx, &b)| {
      let offset = (b as u8 - 'a' as u8) as usize;
      if max_offset < offset {
        max_offset = offset;
      }
      idxs[offset].push(idx);
    });

    let mut target_idxs: Vec<usize> = idxs[max_offset].clone();
    let mut offset: usize = 1;

    // println!("target_idxs {:?}", target_idxs);
    while target_idxs.len() > 1 {
      let mut keep: Vec<usize> = Vec::new();
      let mut max_b: u8 = 0 as u8;
      (0..target_idxs.len()).for_each(|idx| {
        if target_idxs[idx] + offset >= s.len() {
          return;
        }
        if bb[target_idxs[idx] + offset] > max_b {
          keep.clear();
          keep.push(target_idxs[idx]);
          max_b = bb[target_idxs[idx] + offset];
        } else if bb[target_idxs[idx] + offset] == max_b {
          keep.push(target_idxs[idx]);
        }
      });

      // println!("keep {:?}", keep);
      target_idxs = keep;

      offset += 1;
    }
    // println!("{:?}", target_idxs);
    String::from_utf8(bb.drain(target_idxs[0]..).collect()).unwrap()
  }
}

fn main() {
  println!("{}", Solution::last_substring("leetcode".to_string()));
  println!("{}", Solution::last_substring("abab".to_string()));
}
