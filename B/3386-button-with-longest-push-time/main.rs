impl Solution {
  pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
    let mut ans : i32 = 0;
    let mut max : i32 = 0;
    let mut base : i32 = 0;
    events.iter().for_each(|evt | {
      if evt[1]-base > max {
        max = evt[1] - base;
        ans = evt[0];
      } else if evt[1]-base ==  max && evt[0] < ans{
        ans = evt[0];
      }
      base =evt[1];
    });
    ans
  }
}
