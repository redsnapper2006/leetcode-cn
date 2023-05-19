struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn num_tile_possibilities(tiles: String) -> i32 {
    let buf: Vec<u8> = tiles.as_bytes().to_vec();
    let mut set: HashSet<String> = HashSet::new();
    let mut container: Vec<Vec<u8>> = Vec::new();
    container.push(Vec::new());
    (0..buf.len()).for_each(|idx| {
      let size = container.len();
      (0..size).for_each(|candi| {
        let offset = container[candi].len();
        (0..offset + 1).for_each(|i| {
          let mut new = container[candi].clone();
          new.insert(i, buf[idx]);
          container.push(new);
        });
      });
    });

    container.iter().for_each(|c| {
      set.insert(String::from_utf8(c.clone()).unwrap());
    });

    set.len() as i32 - 1
  }
}

fn main() {
  println!("{}", Solution::num_tile_possibilities("AAB".to_string()));

  println!("{}", Solution::num_tile_possibilities("AAABBC".to_string()));

  println!("{}", Solution::num_tile_possibilities("V".to_string()));
}
