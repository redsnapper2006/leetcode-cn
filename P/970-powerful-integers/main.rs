struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
    if bound <= 1 {
      return Vec::new();
    }

    let mut xs: Vec<i32> = vec![1];
    if x > 1 {
      let mut xx = x;
      while xx < bound {
        xs.push(xx);
        xx *= x;
      }
    }
    let mut ys: Vec<i32> = vec![1];
    if y > 1 {
      let mut yy = y;
      while yy < bound {
        ys.push(yy);
        yy *= y;
      }
    }

    let mut set: HashSet<i32> = HashSet::new();
    let mut yidx: usize = ys.len();
    (0..xs.len()).for_each(|xidx| {
      let mut ysidx: usize = 0;
      while ysidx < yidx {
        if xs[xidx] + ys[ysidx] <= bound {
          set.insert(xs[xidx] + ys[ysidx]);
          ysidx += 1;
        } else {
          yidx = ysidx;
          break;
        }
      }
    });

    set.into_iter().collect::<Vec<i32>>()
  }
}
