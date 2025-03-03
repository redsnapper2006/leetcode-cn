struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn palindrome_partition(s: String, k: i32) -> i32 {
    fn make_pal(bb: &Vec<u8>, start: usize, end: usize) -> i32 {
      let mut s = start as i32;
      let mut e = end as i32;
      let mut ans: i32 = 0;
      while s <= e {
        if bb[s as usize] != bb[e as usize] {
          ans += 1;
        }
        s += 1;
        e -= 1;
      }
      ans
    }

    let mut m: HashMap<usize, HashMap<i32, i32>> = HashMap::new();
    let bb = s.as_bytes().to_vec();
    fn dfs(bb: &Vec<u8>, index: usize, k: i32, m: &mut HashMap<usize, HashMap<i32, i32>>) -> i32 {
      if m.contains_key(&index) && m.get(&index).unwrap().contains_key(&k) {
        return *m.get(&index).unwrap().get(&k).unwrap();
      }

      if k == 1 {
        let ans = make_pal(bb, index, bb.len() - 1);
        m.entry(index)
          .and_modify(|x| {
            x.insert(k, ans);
          })
          .or_insert(HashMap::from([(k, ans)]));
        return ans;
      }

      let mut ans: i32 = (bb.len() - index) as i32 - k;
      (index..(bb.len() - k as usize + 1)).for_each(|idx| {
        ans = ans.min(make_pal(bb, index, idx) + dfs(bb, idx + 1, k - 1, m));
      });
      m.entry(index)
        .and_modify(|x| {
          x.insert(k, ans);
        })
        .or_insert(HashMap::from([(k, ans)]));
      ans
    }
    dfs(&bb, 0, k, &mut m);
    *m.get(&0).unwrap().get(&k).unwrap()
  }
}

fn main() {
  println!("{}", Solution::palindrome_partition("abc".to_string(), 2));
  println!("{}", Solution::palindrome_partition("aabbc".to_string(), 3));
  println!(
    "{}",
    Solution::palindrome_partition("leetcode".to_string(), 8)
  );
}
