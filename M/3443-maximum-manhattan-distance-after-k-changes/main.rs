impl Solution {
  pub fn max_distance(s: String, k: i32) -> i32 {
    let (mut n_cnt, mut s_cnt, mut e_cnt, mut w_cnt): (i32, i32, i32, i32) = (0, 0, 0, 0);

    let mut ans: i32 = 0;
    s.as_bytes().iter().for_each(|&b| {
      match b {
        b'N' => n_cnt += 1,
        b'S' => s_cnt += 1,
        b'E' => e_cnt += 1,
        _ => w_cnt += 1,
      };

      let v_d: i32 = n_cnt.min(s_cnt).min(k);
      let h_d: i32 = w_cnt.min(e_cnt).min(k - v_d);
      let steps = n_cnt.max(s_cnt) - n_cnt.min(s_cnt) + v_d * 2 + w_cnt.max(e_cnt)
        - w_cnt.min(e_cnt)
        + h_d * 2;
      ans = ans.max(steps);
    });

    ans
  }
}
