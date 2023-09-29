struct Solution {}

impl Solution {
  pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut sum: i32 = 0;
    let mut prev: i32 = -2;
    for i in 0..flowerbed.len() {
      if flowerbed[i] == 1 {
        sum += (i as i32 - prev - 2) / 2;
        prev = i as i32;
      }
    }
    sum += (flowerbed.len() as i32 - prev - 1) / 2;
    sum >= n
  }
}
