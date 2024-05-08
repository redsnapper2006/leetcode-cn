impl Solution {
  pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
    plants
      .iter()
      .enumerate()
      .fold((capacity, 0), |(remain, steps), (i, &plant)| {
        match remain >= plant {
          true => (remain - plant, steps + 1),
          false => (capacity - plant, steps + (i as i32 + 1) * 2 - 1),
        }
      })
      .1
  }
}
