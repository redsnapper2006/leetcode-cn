impl Solution {
  pub fn string_hash(s: String, k: i32) -> String {
    let k = k as usize;
    let bb = s.as_bytes().to_vec();
    let mut ans : Vec<u8> = vec![];
    (0..bb.len()).step_by(k).for_each(|idx| {
      let mut sum : i32 = 0;
      (idx..idx+k).for_each(|ii | {
        sum += (bb[ii] - b'a') as i32;
      });
      ans.push( b'a' + (sum % 26) as u8 );
    });

    String::from_utf8(ans).unwrap()
  }
}

struct Solution {}
fn main() {
  println!("{}", Solution::string_hash("abcd".to_string(), 2));
}
