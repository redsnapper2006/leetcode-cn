struct Solution {}

impl Solution {
  pub fn capitalize_title(title: String) -> String {
    title
      .split(" ")
      .map(|v| match v.len() {
        1 => v.to_lowercase(),
        2 => v.to_lowercase(),
        _ => {
          let v = v.to_lowercase();
          let mut c = v.chars();
          match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
          }
        }
      })
      .collect::<Vec<String>>()
      .join(" ")
  }
}
