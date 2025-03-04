struct Solution {}

impl Solution {
  pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let mut meetings = meetings;
    meetings.sort_unstable();
    let mut s: i32 = 0;
    let mut e: i32 = 0;
    let mut sum: i32 = 0;
    meetings.iter().for_each(|meet| {
      if meet[0] > e {
        if e != 0 && s != 0 {
          sum += e - s + 1;
        }
        s = meet[0];
        e = meet[1];
      } else if meet[1] > e {
        e = meet[1];
      }
    });
    sum += e - s + 1;
    days - sum
  }
}

fn main() {
  println!(
    "{}",
    Solution::count_days(1000000000, vec![vec![1, 1000000000]])
  );
}
