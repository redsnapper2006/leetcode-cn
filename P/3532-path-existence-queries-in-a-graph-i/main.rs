impl Solution {
  pub fn path_existence_queries(
    n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>,
  ) -> Vec<bool> {
    let mut colors: Vec<i32> = vec![0; n as usize];
    let mut color: i32 = 0;
    for i in 1..nums.len() {
      if nums[i] - nums[i - 1] > max_diff {
        color += 1;
      }
      colors[i] = color;
    }

    queries
      .iter()
      .map(|q| colors[q[0] as usize] == colors[q[1] as usize])
      .collect::<Vec<bool>>()
  }
}
