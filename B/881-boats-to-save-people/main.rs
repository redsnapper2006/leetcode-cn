struct Solution {}

impl Solution {
  pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut people = people;
    people.sort_unstable();

    let mut ans: i32 = 0;
    let mut start: usize = 0;
    let mut end: usize = people.len() - 1;
    while start < end {
      if people[start] + people[end] <= limit {
        start += 1;
      }
      end -= 1;

      ans += 1;
    }
    if start == end {
      ans += 1;
    }
    ans
  }
}
