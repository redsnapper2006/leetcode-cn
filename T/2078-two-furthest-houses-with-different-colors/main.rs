impl Solution {
  pub fn max_distance(colors: Vec<i32>) -> i32 {
    let mut l: usize = 0;
    while colors[l] == colors[colors.len() - 1] {
      l += 1;
    }
    let mut r: usize = colors.len() - 1;
    while colors[r] == colors[0] {
      r -= 1;
    }
    r.max(colors.len() - 1 - l) as _
  }

  pub fn max_distance2(colors: Vec<i32>) -> i32 {
    let mut mn1: (i32, i32) = (-1, -1);
    let mut mn2: (i32, i32) = (-1, -1);
    let mut mx1: (i32, i32) = (-1, -1);
    let mut mx2: (i32, i32) = (-1, -1);

    for idx in 0..colors.len() {
      if mn1.0 == -1 {
        mn1 = (colors[idx], idx as i32);
      } else if mn1.0 != colors[idx] && mn2.0 == -1 {
        mn2 = (colors[idx], idx as i32);
      }

      if mx1.0 == -1 {
        mx1 = (colors[colors.len() - 1 - idx], (colors.len() - 1 - idx) as i32);
      } else if mx1.0 != colors[colors.len() - 1 - idx] && mx2.0 == -1 {
        mx2 = (colors[colors.len() - 1 - idx], (colors.len() - 1 - idx) as i32);
      }
    }

    let mut ans: i32 = -1;
    ans = ans.max(if mx1.0 != mn1.0 { mx1.1 - mn1.1 } else { -1 }).max(if mx2.0 != mn1.0 {
      mx2.1 - mn1.1
    } else {
      -1
    });
    ans = ans.max(if mx1.0 != mn2.0 { mx1.1 - mn2.1 } else { -1 }).max(if mx2.0 != mn2.0 {
      mx2.1 - mn2.1
    } else {
      -1
    });
    ans
  }
}
