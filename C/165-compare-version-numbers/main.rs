impl Solution {
  pub fn compare_version(version1: String, version2: String) -> i32 {
    fn s2d(ver: String) -> Vec<i32> {
      let bb = ver.as_bytes().to_vec();
      let mut idx: usize = 0;
      let mut v: i32 = 0;
      let mut ans: Vec<i32> = vec![];
      while idx < bb.len() {
        if bb[idx] == b'.' {
          ans.push(v);
          v = 0;
        } else {
          v = v * 10 + (bb[idx] - b'0') as i32;
        }
        idx += 1;
      }
      ans.push(v);
      ans
    }

    let mut d1: Vec<i32> = s2d(version1);
    let mut d2: Vec<i32> = s2d(version2);

    let s = d1.len().max(d2.len());
    for _ in d1.len()..s {
      d1.push(0);
    }
    for _ in d2.len()..s {
      d2.push(0);
    }

    let mut idx: usize = 0;
    while idx < s {
      if d1[idx] < d2[idx] {
        return -1;
      } else if d1[idx] > d2[idx] {
        return 1;
      }
      idx += 1;
    }

    0
  }
}
