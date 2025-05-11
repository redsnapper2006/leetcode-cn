impl Solution {
  pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    let mut idx: usize = 0;
    let mut b: Vec<bool> = vec![false; 3];
    while idx < arr.len() {
      b[idx % 3] = arr[idx] % 2 == 1;
      if b[0] && b[1] && b[2] {
        return true;
      }
      idx += 1;
    }
    false
  }
}
