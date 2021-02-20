struct Solution {}

impl Solution {
  pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut asc: Vec<i32> = vec![0; ratings.len()];
    let mut desc: Vec<i32> = vec![0; ratings.len()];
    for i in 0..ratings.len() {
      if i > 0 && ratings[i] > ratings[i - 1] {
        asc[i] = asc[i - 1] + 1;
      }
      if i > 0 && ratings[ratings.len() - 1 - i] > ratings[ratings.len() - 1 - i + 1] {
        desc[ratings.len() - 1 - i] = desc[ratings.len() - 1 - i + 1] + 1;
      }
    }

    let mut sum = 0;
    for i in 0..ratings.len() {
      if asc[i] > 0 && desc[i] > 0 {
        let mut c = asc[i];
        if c < desc[i] {
          c = desc[i];
        }
        sum += c + 1;
      }
      if asc[i] > 0 && desc[i] == 0 {
        sum += asc[i] + 1;
      }
      if asc[i] == 0 && desc[i] > 0 {
        sum += desc[i] + 1;
      }
      if asc[i] == 0 && desc[i] == 0 {
        sum += 1;
      }
    }
    sum
  }
}

fn main() {
  // println!("{}",Solution::candy(vec![0;5]));
  println!("{:?}", vec![0; 5]);
}
