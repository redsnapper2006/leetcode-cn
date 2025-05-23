struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn num_rabbits(mut answers: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    answers.iter().for_each(|&v| {
      m.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });
    let mut ans: i32 = 0;
    for (k, v) in m {
      let mut v = v;
      while v > 0 {
        v -= k + 1;
        ans += k + 1;
      }
    }
    ans
  }

  pub fn num_rabbits2(mut answers: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    answers.sort();

    let mut p = -1;
    let mut sum = 0;
    for c in answers {
      if c == 0 {
        sum += 1;
        continue;
      }
      if !m.contains_key(&c) {
        sum += c + 1;
        m.insert(c, c);
      } else {
        let mut v = m.get_mut(&c).unwrap();
        *v -= 1;
        if *v == 0 {
          m.remove(&c);
        }
      }
    }
    sum
  }
}

fn main() {
  println!("{}", Solution::num_rabbits(vec![0; 5]));
}
