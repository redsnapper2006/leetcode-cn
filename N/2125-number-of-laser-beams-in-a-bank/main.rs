impl Solution {
  pub fn number_of_beams(bank: Vec<String>) -> i32 {
    bank
      .iter()
      .fold((0, 0), |(ans, prev), b| {
        let c = b.chars().filter(|&x| x == '1').count() as i32;
        (ans + prev * c, if c == 0 { prev } else { c })
      })
      .0
  }
}
