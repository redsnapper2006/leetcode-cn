impl Solution {
  pub fn push_dominoes(dominoes: String) -> String {
    let mut b = format!("L{}R", dominoes).as_bytes().to_vec();
    let mut prev: usize = 0;
    (1..b.len()).for_each(|idx| {
      if b[idx] == b'.' {
        return;
      }
      if b[idx] == b[prev] {
        (prev..=idx).for_each(|i| {
          b[i] = b[prev];
        });
      } else if b[idx] == b'L' {
        (prev + 1..(idx + prev + 1) / 2).for_each(|i| {
          b[i] = b'R';
        });
        ((idx + prev) / 2 + 1..idx).for_each(|i| {
          b[i] = b'L';
        });
      }
      prev = idx;
    });
    String::from_utf8(b[1..b.len() - 1].to_vec()).unwrap()
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::push_dominoes("RR.L".to_string()));
}
