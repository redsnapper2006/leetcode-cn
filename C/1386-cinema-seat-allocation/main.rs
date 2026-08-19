use std::collections::HashMap;
impl Solution {
  pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    let mut buf: HashMap<i32, u8> = HashMap::new();
    for rs in &reserved_seats {
      if rs[1] < 2 || rs[1] > 9 {
        continue;
      }
      *buf.entry(rs[0]).or_insert(0) |= 1 << (rs[1] - 2);
    }

    let mut ans: i32 = (n - buf.len() as i32) << 1;
    for &v in buf.values() {
      if v & 0x0F == 0 || v & 0x3C == 0 || v & 0xF0 == 0 {
        ans += 1;
      }
    }
    ans
  }
}
