impl Solution {
  pub fn survived_robots_healths(
    positions: Vec<i32>, healths: Vec<i32>, directions: String,
  ) -> Vec<i32> {
    let mut buf: Vec<(i32, i32, u8, usize)> = vec![];
    let bb = directions.as_bytes().to_vec();
    for i in 0..positions.len() {
      buf.push((positions[i], healths[i], bb[i], i));
    }
    buf.sort_unstable();

    let mut stack: Vec<(i32, i32, u8, usize)> = vec![];
    for i in 0..buf.len() {
      if stack.len() == 0
        || stack[stack.len() - 1].2 == buf[i].2
        || stack[stack.len() - 1].2 == b'L' && buf[i].2 == b'R'
      {
        stack.push(buf[i]);
        continue;
      }
      let mut t = buf[i].clone();
      while stack.len() > 0 && stack[stack.len() - 1].2 == b'R' && stack[stack.len() - 1].1 < t.1 {
        stack.pop();
        t.1 -= 1;
      }

      if stack.len() > 0 && stack[stack.len() - 1].2 == b'R' {
        let last = stack.len() - 1;
        if stack[last].1 == t.1 {
          stack.pop();
        } else {
          stack[last].1 -= 1;
        }
      } else {
        stack.push(t);
      }
    }
    stack.sort_by(|x, y| x.3.cmp(&y.3));

    stack.iter().map(|x| x.1).collect::<Vec<i32>>()
  }
}
