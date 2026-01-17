impl Solution {
  pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
    let mut l: i32 = 0;
    for i in 0..bottom_left.len() {
      let tr1 = &top_right[i];
      let bl1 = &bottom_left[i];
      for j in i + 1..bottom_left.len() {
        let tr2 = &top_right[j];
        let bl2 = &bottom_left[j];
        let w = tr1[0].min(tr2[0]) - bl1[0].max(bl2[0]);
        let h = tr1[1].min(tr2[1]) - bl1[1].max(bl2[1]);
        l = l.max(w.min(h));
      }
    }
    l as i64 * l as i64
  }
}
