impl Solution {
  pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let n = stone_value.len();
    let mut sum: i32 = 0;

    let mut f1: i32 = 0;
    let mut f2: i32 = i32::MAX;
    let mut f3: i32 = i32::MAX;
    for i in (0..n).rev() {
      sum += stone_value[i];
      let cur = sum - f1.min(f2).min(f3);
      f3 = f2;
      f2 = f1;
      f1 = cur;
    }
    if f1 * 2 == sum {
      "Tie".to_string()
    } else if f1 * 2 > sum {
      "Alice".to_string()
    } else {
      "Bob".to_string()
    }
  }
}
