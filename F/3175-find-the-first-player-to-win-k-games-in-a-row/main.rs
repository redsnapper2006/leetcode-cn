impl Solution {
  pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
    let mut win: i32 = 0;
    let mut idx: usize = 0;
    let mut n: usize = 1;
    while n < skills.len() {
      if skills[n] > skills[idx] {
        idx = n;
        win = 0;
      }
      win += 1;
      if win == k {
        break;
      }
      n += 1;
    }

    idx as i32
  }
}
