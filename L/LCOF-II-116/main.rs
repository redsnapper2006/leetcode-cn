impl Solution {
  pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<i32> = vec![0; is_connected.len()];

    fn dfs(idx: usize, buf: &mut Vec<i32>, is_connected: &Vec<Vec<i32>>, color: i32) {
      if buf[idx] != 0 {
        return;
      }

      buf[idx] = color;
      for (ii, v) in is_connected[idx].iter().enumerate() {
        if ii == idx || *v == 0 {
          continue;
        }
        dfs(ii, buf, is_connected, color);
      }
    }

    let mut color: i32 = 0;
    (0..is_connected.len()).for_each(|v| {
      if buf[v] != 0 {
        return;
      }
      color += 1;
      dfs(v, &mut buf, &is_connected, color);
    });
    color
  }
}
