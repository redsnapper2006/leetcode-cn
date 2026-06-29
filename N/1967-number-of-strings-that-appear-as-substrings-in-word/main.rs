impl Solution {
  pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
    let wb = word.as_bytes().to_vec();
    patterns.iter().fold(0, |ans, pat| {
      let pb = pat.as_bytes().to_vec();
      if wb.len() < pb.len() {
        return ans;
      }

      for i in 0..=(wb.len() - pb.len()) {
        let mut valid = true;
        for j in 0..pb.len() {
          if pb[j] != wb[i + j] {
            valid = false;
            break;
          }
        }
        if valid {
          return ans + 1;
        }
      }

      ans
    })
  }
}
