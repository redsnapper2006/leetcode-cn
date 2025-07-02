impl Solution {
  pub fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
    let mut score: i64 = 0;
    let mut visit: Vec<bool> = vec![false; instructions.len()];

    let mut idx: i32 = 0;
    while idx < instructions.len() as i32 {
      if visit[idx as usize] {
        break;
      }
      visit[idx as usize] = true;
      if instructions[idx as usize] == "add" {
        score += values[idx as usize] as i64;

        idx += 1;
      } else {
        idx += values[idx as usize];
        if idx < 0 || idx >= instructions.len() as i32 {
          break;
        }
      }
    }

    score
  }
}
