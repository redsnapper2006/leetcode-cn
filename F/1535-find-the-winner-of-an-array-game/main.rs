impl Solution {
  pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    let mut cnt: i32 = 0;
    let mut max: i32 = arr[0];
    for i in 1..arr.len() {
      if arr[i] > max {
        max = arr[i];
        cnt = 1;
      } else {
        cnt += 1;
      }
      if cnt == k {
        return max;
      }
    }
    max
  }
}