impl Solution {
  pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
    let mut sk2: Vec<i64> = vec![0; skill.len() + 1];
    for i in 0..skill.len() {
      sk2[i + 1] = sk2[i] + skill[i] as i64;
    }
    let mut buf: Vec<i64> = vec![0; skill.len() + 1];
    mana.iter().for_each(|&m| {
      let mut mx: i64 = 0;
      for i in (1..sk2.len()).rev() {
        mx = mx.max(buf[i] + (sk2[sk2.len() - 1] - sk2[i - 1]) * m as i64);
      }
      for i in (1..sk2.len()).rev() {
        buf[i] = mx - (sk2[sk2.len() - 1] - sk2[i]) * m as i64;
      }
    });
    buf[buf.len() - 1]
  }
}
