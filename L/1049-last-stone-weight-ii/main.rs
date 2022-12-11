struct Solution {}

impl Solution {
  pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let sum: i32 = stones.iter().sum();
    let half = sum / 2;
    let mut dp: Vec<bool> = vec![false; half as usize + 1];
    dp[0] = true;
    for stone in stones {
      for i in (0..=half as usize).rev() {
        if !dp[i] || i as i32 + stone > half {
          continue;
        }
        dp[i + stone as usize] = true;
      }
    }
    for i in (0..=half as usize).rev() {
      if dp[i] {
        return sum - 2 * i as i32;
      }
    }
    0
  }

  pub fn last_stone_weight_ii2(stones: Vec<i32>) -> i32 {
    let sum: i32 = stones.iter().sum();
    let half = sum / 2;
    let mut dp: Vec<bool> = vec![false; half as usize + 1];
    dp[0] = true;
    for stone in stones {
      for j in (stone as usize..=half as usize).rev() {
        dp[j as usize] = dp[j as usize] || dp[j as usize - stone as usize];
      }
    }
    for i in (0..=half as usize).rev() {
      if dp[i] {
        return sum - 2 * i as i32;
      }
    }
    0
  }
}

fn main() {
  println!(
    "{:}",
    Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1])
  );
  println!(
    "{:}",
    Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40])
  );

  println!(
    "{:}",
    Solution::last_stone_weight_ii(vec![
      89, 23, 100, 93, 82, 98, 91, 85, 33, 95, 72, 98, 63, 46, 17, 91, 92, 72, 77, 79, 99, 96, 55,
      72, 24, 98, 79, 93, 88, 92
    ])
  );
}
