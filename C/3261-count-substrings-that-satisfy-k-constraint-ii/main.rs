struct Solution {}

impl Solution {
  pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let bb = s.as_bytes().to_vec();
    let k = k as i64;
    let mut lp: Vec<usize> = vec![0; bb.len()];
    let mut sum: Vec<i64> = vec![0; bb.len() + 1];
    let mut left: usize = 0;
    let mut right: usize = 0;

    let mut zero: i64 = 0;
    let mut one: i64 = 0;
    while right < bb.len() {
      if bb[right] == b'0' {
        zero += 1;
      } else {
        one += 1;
      }
      right += 1;

      while zero > k && one > k {
        if bb[left] == b'0' {
          zero -= 1;
        } else {
          one -= 1;
        }
        left += 1;
      }
      lp[right - 1] = left;
      sum[right] = sum[right - 1] + (right - left) as i64;
    }

    let mut ans: Vec<i64> = Vec::new();
    queries.iter().for_each(|query| {
      let s = query[0] as usize;
      let e = query[1] as usize;

      let j = s
        + match lp[s..e + 1].binary_search(&s) {
          Ok(v) => v,
          Err(v) => v,
        };
      let j = j as i64;
      let s = s as i64;

      ans.push(sum[e + 1] - sum[j as usize] + (j - s + 1) * (j - s) / 2);
    });

    ans
  }
}
