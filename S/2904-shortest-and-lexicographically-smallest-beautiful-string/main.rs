impl Solution {
  pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
    let bb = s.as_bytes();
    let (mut start, mut end) = (0usize, 0usize);
    let (mut mn, mut val, mut cnt) = (u128::MAX, 0u128, 0i32);

    while end < bb.len() {
      val = (val << 1) + (bb[end] & 1) as u128;
      cnt += (bb[end] & 1) as i32;
      if cnt == k {
        mn = mn.min(val);
      }

      while  cnt >= k {
        val &= (1u128 << (end - start)) - 1;
        cnt -= (bb[start] & 1) as i32;
        start += 1;
      }
      end += 1;
    }

    if mn == u128::MAX {
      "".to_string()
    } else {
      format!("{mn:b}")
    }
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::shortest_beautiful_substring("100011001".to_string(), 3)
  );

  println!(
    "{}",
    Solution::shortest_beautiful_substring("1011".to_string(), 2)
  );

  println!(
    "{}",
    Solution::shortest_beautiful_substring("1100001110111100100".to_string(), 8)
  );
}
