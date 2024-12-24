struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
    let mut bh: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    let mut ans: i32 = 0;

    (0..apples.len()).for_each(|day| {
      if apples[day] > 0 {
        bh.push(Reverse((day as i32 + days[day], apples[day])));
      }
      while bh.len() > 0 {
        let candi = bh.pop().unwrap().0;
        if candi.0 > day as i32 {
          let r = candi.1;
          if r > 1 {
            bh.push(Reverse((candi.0, r - 1)));
          }
          ans += 1;
          break;
        }
      }
    });
    let mut day = apples.len() as i32;
    while bh.len() > 0 {
      let candi = bh.pop().unwrap().0;
      if candi.0 > day {
        let r = candi.1;
        if r > 1 {
          bh.push(Reverse((candi.0, r - 1)));
        }
        ans += 1;
        day += 1;
      }
    }

    ans
  }
}

fn main() {
  println!(
    "{}",
    Solution::eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2])
  );
  println!(
    "{}",
    Solution::eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2])
  );
}
