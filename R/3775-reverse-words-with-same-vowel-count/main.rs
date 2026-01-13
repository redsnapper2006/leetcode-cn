impl Solution {
  pub fn reverse_words(s: String) -> String {
    let mut bb: Vec<String> = s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
    fn calc(ss: &String) -> i32 {
      let mut cnt: i32 = 0;
      for b in ss.chars() {
        if b == 'a' || b == 'e' || b == 'i' || b == 'o' || b == 'u' {
          cnt += 1;
        }
      }
      cnt
    }
    let mut base: i32 = calc(&bb[0]);

    for i in 1..bb.len() {
      if calc(&bb[i]) == base {
        bb[i] = bb[i].chars().rev().collect::<String>();
      }
    }
    bb.join(" ")
  }
}
