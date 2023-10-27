struct Solution {}

impl Solution {
  pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let mut hc = horizontal_cuts;
    hc.push(h);
    hc.push(0);
    let mut vc = vertical_cuts;
    vc.push(w);
    vc.push(0);
    hc.sort();
    vc.sort();

    let mut max_h = 0;
    let mut max_v = 0;
    for i in 1..hc.len() {
      if hc[i] - hc[i - 1] > max_h {
        max_h = hc[i] - hc[i - 1];
      }
    }
    for i in 1..vc.len() {
      if vc[i] - vc[i - 1] > max_v {
        max_v = vc[i] - vc[i - 1];
      }
    }

    (((max_h as i64) * (max_v as i64)) % 1000000007) as i32
  }
}
