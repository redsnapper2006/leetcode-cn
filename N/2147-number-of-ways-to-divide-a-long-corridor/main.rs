use std::collections::HashMap;
impl Solution {
  pub fn number_of_ways(corridor: String) -> i32 {
    let mut prev: i64 = 1;
    let mut cur: i64 = 1;
    let mut cnt: i64 = 0;
    let mut reset: bool = true;
    for &b in corridor.as_bytes() {
      if b == b'S' {
        cnt += 1;
      }
      if cnt == 0 {
        continue;
      }

      if cnt % 2 == 0 {
        cur += prev;
        cur %= 1000000007;
        reset = true;
      } else if reset {
        prev = cur;
        cur = 0;
        reset = false;
      }
    }

    if cnt > 0 && cnt % 2 == 0 {
      prev as _
    } else {
      0
    }
  }

  pub fn number_of_ways2(corridor: String) -> i32 {
    let mut m: HashMap<i64, i64> = HashMap::new();
    m.insert(0, 1);
    let mut cnt: i64 = 0;
    for &b in corridor.as_bytes() {
      if b == b'S' {
        cnt += 1;
      }

      if cnt > 0 && cnt % 2 == 0 {
        let prev = *m.get(&(cnt - 2)).unwrap();
        let v = m.entry(cnt).or_insert(0);
        *v += prev;
        *v %= 1000000007;
      }
    }

    if cnt > 0 && cnt % 2 == 0 {
      *m.get(&(cnt - 2)).unwrap() as _
    } else {
      0
    }
  }
}
