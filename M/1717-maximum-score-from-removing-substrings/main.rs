impl Solution {
  pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    let mut stack: Vec<u8> = vec![];
    let bb = s.as_bytes().to_vec();

    let base: u8 = if x > y { b'b' } else { b'a' };
    let prev: u8 = if x > y { b'a' } else { b'b' };
    let mx = x.max(y);
    let mn = x.min(y);

    let mut sum: i32 = 0;
    for b in bb.iter() {
      stack.push(*b);
      if *b == base && stack.len() > 1 && stack[stack.len() - 2] == prev {
        stack.pop();
        stack.pop();
        sum += mx;
      }
    }
    let mut idx: usize = 1;
    while idx < stack.len() {
      if idx > 0 && stack[idx] == prev && stack[idx - 1] == base {
        sum += mn;
        stack.remove(idx);
        stack.remove(idx - 1);
        idx -= 1;
      } else {
        idx += 1;
      }
    }
    sum
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5)
  );
}
