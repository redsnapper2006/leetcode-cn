struct Solution {}

impl Solution {
  pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
    fn calc(w: String) -> (Vec<u8>, Vec<i32>) {
      let (mut bvec, mut cvec): (Vec<u8>, Vec<i32>) = (Vec::new(), Vec::new());

      let bb: Vec<u8> = w.as_bytes().to_vec();
      let mut base: u8 = bb[0];
      let mut cnt: i32 = 1;
      for i in 1..bb.len() {
        if bb[i] != base {
          bvec.push(base);
          cvec.push(cnt);
          base = bb[i];
          cnt = 1;
        } else {
          cnt += 1;
        }
      }
      bvec.push(base);
      cvec.push(cnt);
      (bvec, cvec)
    }

    let (sbvec, scvec) = calc(s);
    let mut ret: i32 = 0;
    for w in words {
      let (wbvec, wcvec) = calc(w);
      if sbvec.len() != wbvec.len() {
        continue;
      }
      let mut is_match: bool = true;
      for i in 0..sbvec.len() {
        if sbvec[i] != wbvec[i] || scvec[i] < wcvec[i] || scvec[i] > wcvec[i] && scvec[i] < 3 {
          is_match = false;
          break;
        }
      }
      if is_match {
        ret += 1;
      }
    }
    ret
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::expressive_words(
      "heeellooo".to_string(),
      vec!["hello".to_string(), "hi".to_string(), "helo".to_string()]
    )
  );

  println!(
    "{:?}",
    Solution::expressive_words("heeellooo".to_string(), vec!["axxxrrzzz".to_string()])
  );

  println!(
    "{:?}",
    Solution::expressive_words(
      "dddiiiinnssssssoooo".to_string(),
      vec![
        "dinnssoo".to_string(),
        "ddinso".to_string(),
        "ddiinnso".to_string(),
        "ddiinnssoo".to_string(),
        "ddiinso".to_string(),
        "dinsoo".to_string(),
        "ddiinsso".to_string(),
        "dinssoo".to_string(),
        "dinso".to_string(),
      ]
    )
  );
}
