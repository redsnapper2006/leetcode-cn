struct Solution {}

impl Solution {
  pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
    let mut cnt: i32 = 0;
    fn r2(cost: &Vec<i32>, idx: usize, cnt: &mut i32) -> i32 {
      if idx >= cost.len() {
        return 0;
      }

      let l = r2(cost, (idx + 1) * 2 - 1, cnt);
      let r = r2(cost, (idx + 1) * 2, cnt);
      *cnt += (r - l).abs();
      l.max(r) + cost[idx]
    }
    r2(&cost, 0, &mut cnt);
    cnt
  }

  pub fn min_increments2(n: i32, cost: Vec<i32>) -> i32 {
    let mut buf: Vec<i32> = vec![0; cost.len()];
    fn r1(cost: &Vec<i32>, idx: usize, sum: i32, buf: &mut Vec<i32>) -> i32 {
      if idx >= cost.len() {
        return 0;
      }

      let l = r1(cost, (idx + 1) * 2 - 1, sum + cost[idx], buf);
      let r = r1(cost, (idx + 1) * 2, sum + cost[idx], buf);
      let m = l.max(r);
      buf[idx] = m + cost[idx];
      buf[idx]
    }

    let max = r1(&cost, 0, 0, &mut buf);

    let mut cnt: i32 = 0;
    fn r2(cost: &Vec<i32>, buf: &Vec<i32>, idx: usize, max: i32, cnt: &mut i32) {
      if idx >= buf.len() {
        return;
      }

      let diff = max - buf[idx];
      *cnt += diff;
      r2(cost, buf, (idx + 1) * 2 - 1, max - cost[idx] - diff, cnt);
      r2(cost, buf, (idx + 1) * 2, max - cost[idx] - diff, cnt);
    }
    r2(&cost, &buf, 0, max, &mut cnt);
    cnt
  }
}
