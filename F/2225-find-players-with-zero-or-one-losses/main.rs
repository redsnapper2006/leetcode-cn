use std::collections::HashMap;
impl Solution {
  pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut winner: HashMap<i32, i32> = HashMap::new();
    let mut loser: HashMap<i32, i32> = HashMap::new();

    matches.iter().for_each(|mat| {
      let win = mat[0];
      let lose = mat[1];
      *winner.entry(win).or_default() += 1;
      *loser.entry(lose).or_default() += 1;
    });
    let mut ans_win: Vec<i32> = Vec::new();

    let mut ans_lose: Vec<i32> = Vec::new();

    winner.iter().for_each(|(k, v)| {
      if !loser.contains_key(k) {
        ans_win.push(*k);
      }
    });
    loser.iter().for_each(|(k, v)| {
      if *v == 1 {
        ans_lose.push(*k);
      }
    });
    ans_win.sort_unstable();
    ans_lose.sort_unstable();
    vec![ans_win, ans_lose]
  }
}
