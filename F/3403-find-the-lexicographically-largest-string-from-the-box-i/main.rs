impl Solution {
  pub fn answer_string(word: String, num_friends: i32) -> String {
    if num_friends == 1 {
      return word;
    }

    let num_friends = num_friends as usize;

    let bb = word.as_bytes().to_vec();
    let mut base: usize = 0;
    let mut base_end = bb.len() - num_friends + 1;
    let mut idx: usize = 1;
    while idx < bb.len() {
      let idx_end = bb.len() - num_friends + 1 + (num_friends - 1).min(idx);
      // println!("base {}", String::from_utf8(bb[base..base_end].to_vec()).unwrap());
      // println!("idx {}", String::from_utf8(bb[idx..idx_end].to_vec()).unwrap());
      let mut i: usize = 0;
      let mut is_bigger: bool = false;
      while base + i < base_end && idx + i < idx_end {
        if bb[base + i] == bb[idx + i] {
          i += 1;
          continue;
        } else if bb[base + i] < bb[idx + i] {
          is_bigger = true;
        }
        break;
      }
      if is_bigger {
        base = idx;
        base_end = idx_end;
      }
      // println!("result {} {}",is_bigger, String::from_utf8(bb[base..base_end].to_vec()).unwrap());

      idx += 1;
    }
    String::from_utf8(bb[base..base_end].to_vec()).unwrap()
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::answer_string("dbca".to_string(), 2));
  println!("{}", Solution::answer_string("gggg".to_string(), 4));
}
