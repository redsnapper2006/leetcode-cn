struct Solution {}

impl Solution {
  pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
    let mut cnt: i32 = 0;
    let mut start: usize = 0;
    let mut end: usize = plants.len() - 1;
    let mut ba: i32 = capacity_a;
    let mut bb: i32 = capacity_b;

    while start <= end {
      if start == end {
        let m = ba.max(bb);
        if m < plants[start] {
          cnt += 1;
        }
        start += 1;
        if end == 0 {
          break;
        }
        end -= 1;
      } else {
        if ba < plants[start] {
          ba = capacity_a;
          cnt += 1;
        }
        ba -= plants[start];
        start += 1;

        if bb < plants[end] {
          bb = capacity_b;
          cnt += 1;
        }
        bb -= plants[end];
        if end == 0 {
          break;
        }
        end -= 1;
      }
    }
    cnt
  }
}
