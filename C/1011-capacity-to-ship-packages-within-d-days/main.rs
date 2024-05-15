impl Solution {
  pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; weights.len()];

    let mut max: i32 = 0;
    let mut sum: i32 = 0;
    (0..weights.len()).for_each(|i| {
      sum += weights[i];
      max = max.max(weights[i]);
      buf[i] = sum;
    });

    let mut start: i32 = max;
    let mut end: i32 = sum;
    while start <= end {
      let capacity: i32 = (start + end) / 2;
      let mut day: i32 = 0;
      let mut sum: i32 = capacity;
      while sum < buf[buf.len() - 1] {
        let ll = match buf.binary_search(&sum) {
          Ok(ov) => ov,
          Err(ev) => ev,
        };

        if buf[ll] == sum {
          sum = buf[ll] + capacity;
        } else {
          if ll == 0 || buf[ll] - buf[ll - 1] > capacity {
            day = days + 1;
            break;
          }
          sum = buf[ll - 1] + capacity;
        }
        day += 1;
      }

      if day < days {
        end = capacity - 1;
      } else {
        start = capacity + 1;
      }
    }
    start
  }
}
