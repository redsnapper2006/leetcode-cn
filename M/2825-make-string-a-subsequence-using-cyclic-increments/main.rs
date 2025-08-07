impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
      let s1 = str1.as_bytes().to_vec();
      let b1 = str2.as_bytes().to_vec();
      let mut b2 = b1.clone();
      for i in 0..b2.len() {
        if b2[i] == b'a' {
          b2[i] = b'z';
        } else {
          b2[i] -= 1;
        }
      }

      let mut i1 : usize = 0;
      let mut i2 : usize = 0;
      while i1 < s1.len() && i2 < b1.len() {
        if s1[i1] == b1[i2] || s1[i1] == b2[i2] {
          i2+=1;
        }

        i1 += 1;
      }
      i2 == b1.len()
    }
}
