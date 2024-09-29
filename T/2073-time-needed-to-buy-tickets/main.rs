struct Solution {}

impl Solution {
  pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut base = tickets[k as usize];
    let mut ans: i32 = 0;
    (0..=k).for_each(|idx| {
      ans += base.min(tickets[idx]);
    });

    base -= 1;
    (k + 1..tickets.len()).for_each(|idx| {
      ans += base.min(tickets[idx]);
    });
    ans
  }
}
