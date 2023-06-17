struct Solution {}

impl Solution {
  pub fn reorganize_string(s: String) -> String {
    let mut bb: Vec<i32> = vec![0; 26];
    s.as_bytes().iter().for_each(|&b| {
      bb[(b - 'a' as u8) as usize] += 1;
    });

    let mut buf = bb
      .into_iter()
      .enumerate()
      .filter(|(_, v)| *v != 0)
      .collect::<Vec<(usize, i32)>>();
    buf.sort_by_key(|(_, v)| -v);

    let size = s.len();
    if size % 2 == 0 && buf[0].1 > size as i32 / 2
      || size % 2 == 1 && buf[0].1 > (size as i32 + 1) / 2
    {
      return "".to_string();
    }

    let mut res: Vec<u8> = vec!['0' as u8; size];
    let mut idx: usize = 0;
    let mut b_idx: usize = 0;
    let mut bb_idx: i32 = 0;
    while idx < size {
      while idx < size && bb_idx < buf[b_idx].1 {
        res[idx] = 'a' as u8 + buf[b_idx].0 as u8;
        idx += 2;
        bb_idx += 1;
      }
      if bb_idx >= buf[b_idx].1 {
        b_idx += 1;
        bb_idx = 0;
      }
      if idx >= size {
        break;
      }
    }

    idx = 1;
    while idx < size {
      while idx < size && bb_idx < buf[b_idx].1 {
        res[idx] = 'a' as u8 + buf[b_idx].0 as u8;
        idx += 2;
        bb_idx += 1;
      }
      if bb_idx >= buf[b_idx].1 {
        b_idx += 1;
        bb_idx = 0;
      }
    }

    String::from_utf8(res).unwrap()
  }
}

fn main() {
  println!("{}", Solution::reorganize_string("aaaaaabbccc".to_string()));
  println!(
    "{}",
    Solution::reorganize_string("ogccckcwmbmxtsbmozli".to_string())
  );
}
