impl Solution {
  pub fn count_collisions(directions: String) -> i32 {
    let mut ans: i32 = 0;
    let dd = directions.as_bytes().to_vec();
    let mut s: i32 = 0;
    let mut e: i32 = dd.len() as i32 - 1;
    while s < dd.len() as i32 && dd[s as usize] == b'L' {
      s += 1;
    }
    while e >= s && dd[e as usize] == b'R' {
      e -= 1;
    }
    while s <= e {
      if dd[s as usize] != b'S' {
        ans += 1;
      }
      s += 1;
    }
    ans
  }

  pub fn count_collisions2(directions: String) -> i32 {
    let mut stack: Vec<u8> = vec![];
    let mut ans: i32 = 0;
    directions.as_bytes().iter().for_each(|&b| {
      if b == b'R' {
        stack.push(b);
      } else if b == b'L' {
        if stack.len() > 0 {
          ans += 1;
          while stack.len() > 0 && stack[stack.len() - 1] == b'R' {
            ans += 1;
            stack.pop();
          }
          stack.push(b'S');
        }
      } else {
        while stack.len() > 0 && stack[stack.len() - 1] == b'R' {
          ans += 1;
          stack.pop();
        }
        stack.push(b);
      }
    });
    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::count_collisions("RRRRLLRRLSSRR".to_string())
  );
}
