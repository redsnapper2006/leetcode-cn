struct Solution {}

impl Solution {
  pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
    let mut ranges = ranges;
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut right: i32 = -1;
    let mut res: i32 = 1;
    ranges.iter().for_each(|range| {
      if range[0] > right {
        res = (res * 2) % 1000000007;
      }

      right = right.max(range[1]);
    });

    res
  }
}

fn main() {
  println!("{}", Solution::count_ways(vec![vec![6, 10], vec![5, 15]]));
}
