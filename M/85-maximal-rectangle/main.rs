impl Solution {
  pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let mut height: Vec<i32> = vec![0; matrix[0].len() + 1];
    let mut ans: i32 = 0;
    for r in 0..matrix.len() {
      let mut stack: Vec<(i32, i32)> = vec![(-1, -1)];
      for c in 0..matrix[0].len() {
        if matrix[r][c] == '0' {
          height[c] = 0;
        } else {
          height[c] += 1;
        }
      }
      for c in 0..height.len() {
        while stack.len() > 1 && stack[stack.len() - 1].0 >= height[c] {
          let (h, i) = stack.pop().unwrap();
          ans = ans.max(h * (c as i32 - stack[stack.len() - 1].1 - 1));
        }
        stack.push((height[c], c as i32));
      }
    }
    ans
  }
}
