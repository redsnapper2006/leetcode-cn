struct Solution {}

impl Solution {
  pub fn min_deletion(nums: Vec<i32>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    let mut cnt: i32 = 0;
    nums.iter().for_each(|&v| {
      stack.push(v);
      if stack.len() % 2 == 0 {
        if stack[stack.len() - 1] == stack[stack.len() - 2] {
          stack.pop();
          cnt += 1;
        }
      }
    });

    match (nums.len() - cnt as usize) % 2 {
      0 => cnt,
      _ => cnt + 1,
    }
  }
}

fn main() {
  println!("{}", Solution::min_deletion(vec![1, 1, 2, 2, 3, 3]));
  println!("{}", Solution::min_deletion(vec![0, 6, 6, 1, 8, 7]));

  println!(
    "{}",
    Solution::min_deletion(vec![
      0, 1, 5, 4, 2, 4, 7, 2, 3, 0, 3, 0, 0, 9, 7, 5, 9, 4, 3, 9, 9, 2, 1, 6, 3, 1, 0, 7, 6, 6, 6,
      0, 1, 7, 1, 9, 4, 9, 3, 3, 4, 5, 0, 3, 8, 7, 1, 8, 4, 5
    ])
  );
}
