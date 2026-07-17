impl Solution {
  pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
    let mut cnt = 0;
    let mut range: Vec<(i32, i32)> = vec![];
    let mut ans = -1;

    for (idx, &a) in arr.iter().enumerate() {
      let Err(mut pos) = range.binary_search_by(|probe| probe.0.cmp(&a)) else {
        continue;
      };

      range.insert(pos, (a, a));
      if pos < range.len() - 1 && range[pos].1 + 1 == range[pos + 1].0 {
        if range[pos + 1].1 - range[pos + 1].0 + 1 == m {
          cnt -= 1;
        }
        range[pos].1 = range[pos + 1].1;
        range.remove(pos + 1);
      }

      if pos > 0 && range[pos - 1].1 + 1 == range[pos].0 {
        if range[pos - 1].1 - range[pos - 1].0 + 1 == m {
          cnt -= 1;
        }
        range[pos - 1].1 = range[pos].1;
        range.remove(pos);
        pos -= 1;
      }

      if range[pos].1 - range[pos].0 + 1 == m {
        cnt += 1;
      }

      if cnt > 0 {
        ans = idx as i32 + 1;
      }
    }

    ans
  }
}
