impl Solution {
  pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
    commands.iter().fold(0, |x, com| match com.as_str() {
      "UP" => x - n,
      "RIGHT" => x + 1,
      "DOWN" => x + n,
      _ => x - 1,
    })
  }
}
