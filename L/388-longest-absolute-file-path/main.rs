struct Solution {}

impl Solution {
  pub fn length_longest_path(input: String) -> i32 {
    let lines = input.split("\n");
    let mut stack: Vec<String> = Vec::new();

    let mut max = 0;
    for l in lines {
      let letters = l.as_bytes();
      let mut idx = 0;
      let mut isFirst = true;
      let mut isFile = false;
      for i in 0..letters.len() {
        if isFirst && letters[i] != '\t' as u8 {
          idx = i;
          isFirst = false;
        }
        if letters[i] == '.' as u8 {
          isFile = true;
        }
      }

      if stack.len() == idx {
        stack.push(l.trim_matches('\t').to_string());
      } else {
        stack[idx] = l.trim_matches('\t').to_string();
        stack.truncate(idx + 1);
      }
      if isFile {
        let mut sum: i32 = 0;
        for i in 0..stack.len() {
          sum += stack[i].len() as i32;
        }
        if sum + stack.len() as i32 - 1 > max {
          max = sum + stack.len() as i32 - 1;
        }
      }
    }
    max
  }
}

fn main() {
  println!(
    "{}",
    Solution::length_longest_path(String::from(
      "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
    ))
  );
}
