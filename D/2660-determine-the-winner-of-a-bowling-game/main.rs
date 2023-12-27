struct Solution {}

impl Solution {
  pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {

    let mut s1 :i32 = 0;
    player1.iter().enumerate().for_each(|(idx, v)| {
      s1 += v;
      if idx > 0 && player1[idx-1] == 10 || idx> 1 && player1[idx-2] == 10 {
        s1 += v;
      }
    });

    let mut s2 :i32 = 0;
    player2.iter().enumerate().for_each(|(idx, v)| {
      s2 += v;
      if idx > 0 && player2[idx-1] == 10 || idx> 1 && player2[idx-2] == 10 {
        s2 += v;
      }
    });

    match s1.cmp(&s2) {
      std::cmp::Ordering::Greater => 1,
      std::cmp::Ordering::Less => 2,
      std::cmp::Ordering::Equal => 0,
    }
  }
}
