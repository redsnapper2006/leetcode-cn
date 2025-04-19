impl Solution {
  pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut ans: i64 = 0;

    nums.iter().for_each(|&v| {
      ans += (nums.partition_point(|x| x <= &(upper - v))
        - nums.partition_point(|x| x < &(lower - v))) as i64;
      if v >= lower - v && v <= upper - v {
        ans -= 1;
      }
    });
    ans / 2
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::count_fair_pairs(vec![0, 0, 0, 0, 0, 0], 0, 0)
  );
  println!(
    "{}",
    Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6)
  );
}
