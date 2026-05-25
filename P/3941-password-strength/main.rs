impl Solution {
  pub fn password_strength(password: String) -> i32 {
    let mut lower: Vec<i32> = vec![0; 26];
    let mut upper: Vec<i32> = vec![0; 26];
    let mut digits: Vec<i32> = vec![0; 10];
    let mut special: Vec<i32> = vec![0; 4];

    password.as_bytes().iter().for_each(|&b| match b {
      b'a'..=b'z' => lower[(b - b'a') as usize] = 1,
      b'A'..=b'Z' => upper[(b - b'A') as usize] = 2,
      b'0'..=b'9' => digits[(b - b'0') as usize] = 3,
      b'!' => special[0] = 5,
      b'@' => special[1] = 5,
      b'#' => special[2] = 5,
      _ => special[3] = 5,
    });
    lower.iter().sum::<i32>() + upper.iter().sum::<i32>() + digits.iter().sum::<i32>() + special.iter().sum::<i32>()
  }
}
