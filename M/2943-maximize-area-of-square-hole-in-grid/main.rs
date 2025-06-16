impl Solution {
  pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
    let mut h_bars = h_bars;
    h_bars.sort_unstable();
    let mut cnt: i32 = 1;
    let mut max_h: i32 = 2;
    (1..h_bars.len()).for_each(|idx| {
      if h_bars[idx] == h_bars[idx - 1] + 1 {
        cnt += 1;
      } else {
        cnt = 1;
      }
      max_h = max_h.max(cnt + 1);
    });

    let mut v_bars = v_bars;
    v_bars.sort_unstable();
    cnt = 1;
    let mut max_v: i32 = 2;
    (1..v_bars.len()).for_each(|idx| {
      if v_bars[idx] == v_bars[idx - 1] + 1 {
        cnt += 1;
      } else {
        cnt = 1;
      }
      max_v = max_v.max(cnt + 1);
    });

    let s = max_v.min(max_h);
    s * s
  }
}
