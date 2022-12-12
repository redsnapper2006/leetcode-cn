struct Solution {}

impl Solution {
  pub fn beauty_sum(s: String) -> i32 {
    let mut ret: i32 = 0;
    let w: Vec<u8> = s.as_bytes().to_vec();
    for (i, c) in w.iter().enumerate() {
      let mut cnt: [i32; 26] = [0; 26];
      let mut max: i32 = 0;
      for j in i..w.len() {
        let offset: i32 = (w[j] - 'a' as u8) as i32;
        cnt[offset as usize] += 1;
        if cnt[offset as usize] > max {
          max = cnt[offset as usize];
        }
        let mut min: i32 = w.len() as i32;
        for m in cnt {
          if m > 0 && m < min {
            min = m;
          }
        }
        ret += max - min;
      }
    }
    ret
  }
}
