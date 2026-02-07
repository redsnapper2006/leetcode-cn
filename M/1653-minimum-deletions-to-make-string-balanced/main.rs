impl Solution {
  pub fn minimum_deletions(s: String) -> i32 {
    let mut b_cnt: i32 = 0;
    let mut ans: i32 = 0;
    s.as_bytes().iter().for_each(|&b| {
      if b == b'b' {
        b_cnt += 1;
      } else {
        ans = (ans + 1).min(b_cnt);
      }
    });
    ans
  }

  pub fn minimum_deletions2(s: String) -> i32 {
    let mut cnt: Vec<(i32, i32)> = vec![(0, 0); s.len()];
    let bb = s.as_bytes().to_vec();
    let mut a_cnt: i32 = 0;
    let mut b_cnt: i32 = 0;
    for i in 0..bb.len() {
      cnt[i].0 += if i == 0 { 0 } else { cnt[i - 1].0 } + if bb[i] == b'b' { 1 } else { 0 };

      cnt[bb.len() - 1 - i].1 +=
        if i == 0 { 0 } else { cnt[bb.len() - i] } + if bb[i] == b'a' { 1 } else { 0 };
    }

    (0..bb.len()).fold(i32::MAX, |ans, idx| ans.min(cnt[idx].0 + cnt[idx].1 - 1))
  }
}

struct Solution {}
