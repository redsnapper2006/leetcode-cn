impl Solution {
  pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut potions = potions.iter().map(|x| *x as i64).collect::<Vec<i64>>();
    potions.sort_unstable();

    spells
      .iter()
      .map(|&spell| {
        let spell = spell as i64;
        (potions.len() - potions.partition_point(|v| *v < (success + spell - 1) / spell)) as i32
      })
      .collect::<Vec<i32>>()
  }
}
