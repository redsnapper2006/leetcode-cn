impl Solution {
  pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    let mut dp: Vec<i32> = vec![0; value as usize];
    nums.iter().for_each(|&n| {
      let mut n = n;
      n %= value;
      if n < 0 {
        n += value;
      }
      dp[n as usize] += 1;
    });
    for i in 0..nums.len() {
      let m = i as i32 % value;
      if (dp[m as usize] - 1) * value + m < i as i32 {
        return i as i32;
      }
    }
    nums.len() as i32
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::find_smallest_integer(vec![1, 3, 5, 7], 2));
}
