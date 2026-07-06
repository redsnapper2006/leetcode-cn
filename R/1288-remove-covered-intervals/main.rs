impl Solution {
  pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.is_empty() {
      return 0;
    }

    let mut sorted = intervals;
    sorted.sort_by(|a, b| a[0].cmp(&b[0]).then_with(|| b[1].cmp(&a[1])));

    let mut end = sorted[0][1];
    let mut covered = 0;
    for interval in sorted.iter().skip(1) {
      if interval[1] <= end {
        covered += 1;
      } else {
        end = interval[1];
      }
    }

    sorted.len() as i32 - covered
  }
}
