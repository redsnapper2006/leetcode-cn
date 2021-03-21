struct Solution {}

impl Solution {
  pub fn remove_duplicate_letters(s: String) -> String {
    let mut visit: Vec<i32> = vec![0; 26];
    let mut stack: Vec<u8> = Vec::new();

    let bb = s.as_bytes().to_vec();
    let mut cnt: Vec<i32> = vec![0; 26];
    for i in 0..bb.len() {
      cnt[(bb[i] - 'a' as u8) as usize] += 1;
    }

    for i in 0..bb.len() {
      let idx = (bb[i] - 'a' as u8) as usize;
      if visit[idx] == 1 {
        cnt[idx] -= 1;
        continue;
      }
      while stack.len() > 0 && stack[stack.len() - 1] > bb[i] {
        let idx = stack[stack.len() - 1] - 'a' as u8;
        if cnt[idx as usize] > 0 {
          visit[idx as usize] = 0;
          stack.remove(stack.len() - 1);
        } else {
          break;
        }
      }
      cnt[idx] -= 1;
      visit[idx] = 1;
      stack.push(bb[i]);
    }
    String::from_utf8(stack).unwrap()
  }

  #[allow(dead_code)]
  pub fn remove_duplicate_letters_2(s: String) -> String {
    let mut buf: Vec<Vec<usize>> = vec![Vec::new(); 26];
    let mut bb: Vec<u8> = s.as_bytes().to_vec();
    for i in 0..bb.len() {
      buf[(bb[i] - 'a' as u8) as usize].push(i);
    }

    loop {
      let mut is_found: bool = false;
      let mut idx: i32 = -1;
      for i in 0..26 {
        if buf[i].len() == 0 {
          continue;
        }
        let candi = buf[i][0];
        let mut is_candi: bool = true;
        for j in 0..26 {
          if j == i || buf[j].len() == 0 {
            continue;
          }
          if buf[j][buf[j].len() - 1] < candi {
            is_candi = false;
            break;
          }
        }
        if is_candi {
          idx = i as i32;
          is_found = true;
          break;
        }
      }
      if !is_found {
        break;
      }

      let offset = buf[idx as usize][0];
      for i in 0..26 {
        if idx == i || buf[i as usize].len() == 0 {
          continue;
        }
        let mut head: i32 = -1;
        for j in 0..buf[i as usize].len() {
          if buf[i as usize][j] > offset {
            head = j as i32;
            break;
          }
        }
        for j in 0..head {
          bb[buf[i as usize][j as usize] as usize] = ' ' as u8;
        }
        buf[i as usize] = buf[i as usize].drain(head as usize..).collect();
      }
      for j in 1..buf[idx as usize].len() {
        bb[buf[idx as usize][j]] = ' ' as u8;
      }
      buf[idx as usize].clear();
    }

    let mut ret: Vec<u8> = Vec::new();
    for i in 0..bb.len() {
      if bb[i] == ' ' as u8 {
        continue;
      }
      ret.push(bb[i]);
    }
    String::from_utf8(ret).unwrap()
  }
}

fn main() {
  println!(
    "{}",
    Solution::remove_duplicate_letters(String::from("cbacdcbc"))
  );
  println!(
    "{}",
    Solution::remove_duplicate_letters(String::from("bccab"))
  );
}
