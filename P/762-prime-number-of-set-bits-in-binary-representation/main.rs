impl Solution {
  pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let prime: Vec<i32> = vec![
      0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0,
    ];

    (left..=right).fold(0, |ans, v| {
      ans + prime[(0..=20).fold((0, v), |(sum, v), _| (sum + v % 2, v / 2)).0 as usize]
    })
  }
}
