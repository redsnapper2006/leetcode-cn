impl Solution {
  pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
    let mut h_bars = h_bars;
    h_bars.sort_unstable();
    let mut v_bars = v_bars;
    v_bars.sort_unstable();
    fn max_bar(bars: &Vec<i32>) -> i32 {
      (1..bars.len())
        .fold((2, 1), |(max_b, cnt), idx| {
          let cnt = if bars[idx] == bars[idx - 1] + 1 {
            cnt + 1
          } else {
            1
          };
          (max_b.max(cnt + 1), cnt)
        })
        .0
    }
    let s = max_bar(&h_bars).min(max_bar(&v_bars));
    s * s
  }
}
