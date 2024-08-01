struct Solution {}

impl Solution {
  pub fn maxmium_score(cards: Vec<i32>, cnt: i32) -> i32 {
    let mut cards = cards;
    cards.sort_unstable_by(|a, b| b.cmp(a));
    let mut sum: i32 = 0;
    let mut idx: usize = 0;
    while idx < cnt as usize {
      sum += cards[idx];
      idx += 1;
    }
    if sum % 2 == 0 {
      return sum;
    }
    idx -= 1;
    let mut left = idx;
    let mut right = idx + 1;
    let mut sum2 = sum;
    let mut found1: bool = false;
    let mut found2: bool = false;
    loop {
      if cards[left] % 2 == 1 {
        sum2 -= cards[left];
        found1 = true;
        break;
      }
      if left == 0 {
        break;
      }
      left -= 1;
    }
    while right < cards.len() {
      if cards[right] % 2 == 0 {
        sum2 += cards[right];
        found2 = true;
        break;
      }
      right += 1;
    }
    let mut ans = 0;
    if found1 && found2 {
      ans = sum2;
    };

    left = idx;
    right = idx + 1;
    sum2 = sum;
    found1 = false;
    found2 = false;
    loop {
      if cards[left] % 2 == 0 {
        sum2 -= cards[left];
        found1 = true;
        break;
      }
      if left == 0 {
        break;
      }
      left -= 1;
    }
    while right < cards.len() {
      if cards[right] % 2 == 1 {
        sum2 += cards[right];
        found2 = true;
        break;
      }
      right += 1;
    }
    if found1 && found2 {
      ans = ans.max(sum2);
    }

    ans
  }
}

fn main() {
  // println!("{}", Solution::maxmium_score(vec![1, 2, 8, 9], 3));
  println!("{}", Solution::maxmium_score(vec![3, 3, 1], 1));
}
