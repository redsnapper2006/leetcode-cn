struct Solution {}

impl Solution {
  pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    queries
      .iter()
      .map(|q| {
        let pb: Vec<u8> = pattern.as_bytes().to_vec();
        let mut qi: usize = 0;
        let mut pi: usize = 0;
        let qb: Vec<u8> = q.as_bytes().to_vec();
        while qi < q.len() && pi < pb.len() {
          if qb[qi] == pb[pi] {
            qi += 1;
            pi += 1;
            continue;
          }
          if qb[qi] >= 'a' as u8 && qb[qi] <= 'z' as u8 {
            qi += 1;
          } else if qb[qi] >= 'A' as u8 && qb[qi] <= 'Z' as u8 {
            return false;
          }
        }
        pi == pb.len()
          && !(qi..q.len())
            .map(|idx| qb[idx] >= 'A' as u8 && qb[idx] <= 'Z' as u8)
            .any(|x| x)
      })
      .collect::<Vec<bool>>()
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::camel_match(
      vec![
        "FooBar".to_string(),
        "FooBarTest".to_string(),
        "FootBall".to_string(),
        "FrameBuffer".to_string(),
        "ForceFeedBack".to_string()
      ],
      "FB".to_string()
    )
  );
  println!(
    "{:?}",
    Solution::camel_match(
      vec![
        "FooBar".to_string(),
        "FooBarTest".to_string(),
        "FootBall".to_string(),
        "FrameBuffer".to_string(),
        "ForceFeedBack".to_string()
      ],
      "FoBa".to_string()
    )
  );
  println!(
    "{:?}",
    Solution::camel_match(
      vec![
        "FooBar".to_string(),
        "FooBarTest".to_string(),
        "FootBall".to_string(),
        "FrameBuffer".to_string(),
        "ForceFeedBack".to_string()
      ],
      "FoBaT".to_string()
    )
  );
}
