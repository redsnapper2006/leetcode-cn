struct Solution {}

impl Solution {
  pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    (1..colors.len() + k as usize - 1)
      .fold((0, 1), |(ans, cnt), idx| {
        if colors[idx % colors.len()] != colors[(idx - 1) % colors.len()] {
          (ans + if cnt + 1 >= k { 1 } else { 0 }, cnt + 1)
        } else {
          (ans, 1)
        }
      })
      .0
  }
}

fn main() {
  println!(
    "{}",
    Solution::number_of_alternating_groups(vec![0, 1, 0, 1], 3)
  );
}
