impl Solution {
  pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
    let mut events = events;
    events.sort_by(|x, y| {
      if x[1] != y[1] {
        x[1].cmp(&y[1])
      } else {
        y[2].cmp(&x[2])
      }
    });
    let mut dp: Vec<(i32, i32)> = vec![];
    for i in 0..events.len() {
      if dp.len() == 0 || dp[dp.len() - 1].0 < events[i][1] {
        dp.push((events[i][1], events[i][2]));
      }
    }
    events.sort_unstable();

    let mut ans: i32 = 0;
    let mut idx: usize = 0;
    let mut prev: i32 = 0;
    for i in 0..events.len() {
      while dp[idx].0 < events[i][0] {
        prev = prev.max(dp[idx].1);
        idx += 1;
      }
      ans = ans.max(events[i][2] + prev);
    }

    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::max_two_events(vec![
      vec![10, 83, 53],
      vec![63, 87, 45],
      vec![97, 100, 32],
      vec![51, 61, 16]
    ])
  );
  println!(
    "{}",
    Solution::max_two_events(vec![
      vec![28, 81, 48],
      vec![27, 90, 94],
      vec![97, 99, 79],
      vec![5, 35, 81],
      vec![65, 94, 84],
      vec![65, 83, 58],
      vec![94, 94, 31],
      vec![39, 52, 73]
    ])
  );
}
