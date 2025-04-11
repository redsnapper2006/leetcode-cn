impl Solution {
  pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    let mut ans: i32 = 0;
    (1..=9).for_each(|c| {
      let v = c * 10 + c;
      if v <= high && v >= low {
        ans += 1;
      }
    });

    (1..=9).for_each(|d1| {
      (0..=9).for_each(|d2| {
        (0..=9).for_each(|d3| {
          (0..=9).for_each(|d4| {
            if d1 + d2 != d3 + d4 {
              return;
            }
            let v = d1 * 1000 + d2 * 100 + d3 * 10 + d4;
            if v <= high && v >= low {
              ans += 1;
            }
          })
        });
      });
    });

    ans
  }
}
