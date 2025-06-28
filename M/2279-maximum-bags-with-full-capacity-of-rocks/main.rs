impl Solution {
  pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
    let mut buf: Vec<i32> = vec![];
    for i in 0..capacity.len() {
      buf.push(capacity[i] - rocks[i]);
    }
    buf.sort_unstable();

    let mut ans: i32 = 0;
    let mut ar = additional_rocks;
    for i in 0..buf.len() {
      if ar >= buf[i] {
        ar -= buf[i];
        ans += 1;
      } else {
        return ans;
      }
    }
    ans
  }
}
