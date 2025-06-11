impl Solution {
  pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
    let mut players = players;
    let mut trainers = trainers;
    players.sort_unstable();
    trainers.sort_unstable();

    let mut idx_p: usize = 0;
    let mut idx_t: usize = 0;
    let mut ans: i32 = 0;
    while idx_p < players.len() && idx_t < trainers.len() {
      if players[idx_p] <= trainers[idx_t] {
        idx_p += 1;
        ans += 1;
      }

      idx_t += 1;
    }
    ans
  }
}
