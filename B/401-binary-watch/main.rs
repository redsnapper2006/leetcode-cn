use std::collections::HashMap;

impl Solution {
  pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let m: HashMap<i32, Vec<i32>> = HashMap::from([
      (0, vec![0]),
      (1, vec![1, 2, 4, 8, 16, 32]),
      (2, vec![3, 5, 6, 9, 10, 12, 17, 18, 20, 24, 33, 34, 36, 40, 48]),
      (
        3,
        vec![
          7, 11, 13, 14, 19, 21, 22, 25, 26, 28, 35, 37, 38, 41, 42, 44, 49, 50, 52, 56,
        ],
      ),
      (4, vec![15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58]),
      (5, vec![31, 47, 55, 59]),
    ]);

    let mut ans: Vec<String> = vec![];
    for (k, v) in m.iter() {
      if !m.contains_key(&(turned_on - k)) {
        continue;
      }
      let mm = m.get(&(turned_on - k)).unwrap();
      for h in v.iter() {
        if *h > 11 {
          continue;
        }
        for mmm in mm.iter() {
          ans.push(format!("{}:{:02}", h, mmm));
        }
      }
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!("{:?}", Solution::read_binary_watch(2));
}
