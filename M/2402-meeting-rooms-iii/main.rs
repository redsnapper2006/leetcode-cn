use std::collections::BTreeSet;

impl Solution {
  pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut meetings = meetings;
    meetings.sort_unstable();
    let mut candidate: BTreeSet<usize> = BTreeSet::new();
    for i in 0..n {
      candidate.insert(i);
    }
    let mut heap: BTreeSet<(i64, usize)> = BTreeSet::new();

    let mut dp: Vec<i32> = vec![0; n];
    meetings.iter().for_each(|meet| {
      while heap.len() > 0 && heap.first().unwrap().0 <= meet[0] as i64 {
        let (_, m_idx) = heap.pop_first().unwrap();
        candidate.insert(m_idx);
      }
      if candidate.len() > 0 {
        let m_idx = candidate.pop_first().unwrap();
        dp[m_idx] += 1;
        heap.insert((meet[1] as i64, m_idx));
      } else {
        let (time, m_idx) = heap.pop_first().unwrap();
        dp[m_idx] += 1;
        heap.insert((time + (meet[1] - meet[0]) as i64, m_idx));
      }
    });

    let mut ans: usize = 0;
    let mut mx: i32 = 0;
    for i in 0..n {
      if dp[i] > mx {
        mx = dp[i];
        ans = i;
      }
    }
    ans as _
  }
}

struct Solution {}
fn main() {
  println!(
    "{}",
    Solution::most_booked(
      2,
      vec![
        vec![52123, 352123],
        vec![52124, 352124],
        vec![52125, 352125],
        vec![52126, 352126]
      ]
    )
  );
}
