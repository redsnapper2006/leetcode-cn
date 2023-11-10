struct Solution {}

impl Solution {
  pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut potions = potions;
    potions.sort();

    spells
      .iter()
      .map(|&s| {
        let mut start: i32 = 0;
        let mut end: i32 = potions.len() as i32 - 1;

        while start <= end {
          let m = (start + (end - start) / 2) as usize;
          if (potions[m] as i64 * s as i64) < success {
            start = m as i32 + 1;
          } else {
            end = m as i32 - 1;
          }
        }
        potions.len() as i32 - start
      })
      .collect::<Vec<i32>>()
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::successful_pairs(vec![3], vec![1, 2, 3, 4, 5], 7)
  );
}
