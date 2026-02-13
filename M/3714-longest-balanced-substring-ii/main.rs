use std::collections::HashMap;
impl Solution {
  pub fn longest_balanced(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut ans: i32 = 0;

    fn c1(bb: &Vec<u8>) -> i32 {
      let mut ans: usize = 0;
      let mut idx: usize = 0;
      while idx < bb.len() {
        let prev = idx;
        idx += 1;
        while idx < bb.len() && bb[idx] == bb[idx - 1] {
          idx += 1;
        }
        ans = ans.max(idx - prev);
      }
      ans as _
    }

    fn c2(bb: &Vec<u8>, x: u8, y: u8) -> i32 {
      let mut idx: usize = 0;
      let mut m: HashMap<i32, i32> = HashMap::new();
      let mut ans: i32 = 0;
      while idx < bb.len() {
        m.clear();
        m.insert(0, idx as i32 - 1);
        let mut diff: i32 = 0;
        while idx < bb.len() && (bb[idx] == x || bb[idx] == y) {
          diff += if bb[idx] == x { 1 } else { -1 };
          if m.contains_key(&diff) {
            ans = ans.max(idx as i32 - m.get(&diff).unwrap());
          } else {
            m.insert(diff, idx as i32);
          }
          idx += 1;
        }
        idx += 1;
      }
      ans
    }

    fn c3(bb: &Vec<u8>) -> i32 {
      let mut idx: usize = 0;
      let mut m: HashMap<(i32, i32), i32> = HashMap::new();
      m.insert((0, 0), -1);
      let mut ans: i32 = 0;
      let (mut a, mut b, mut c) = (0, 0, 0);
      while idx < bb.len() {
        match bb[idx] {
          b'a' => a += 1,
          b'b' => b += 1,
          _ => c += 1,
        };

        let k = (a - b, b - c);
        if m.contains_key(&k) {
          ans = ans.max(idx as i32 - m.get(&k).unwrap());
        } else {
          m.insert(k, idx as i32);
        }
        idx += 1;
      }
      ans
    }

    ans = ans.max(c1(&bb));

    ans = ans.max(c2(&bb, b'a', b'b'));
    ans = ans.max(c2(&bb, b'a', b'c'));
    ans = ans.max(c2(&bb, b'b', b'c'));

    ans = ans.max(c3(&bb));

    ans as _
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::longest_balanced("aabcc".to_string()));
}
