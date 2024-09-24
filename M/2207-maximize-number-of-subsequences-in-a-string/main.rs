struct Solution {}

impl Solution {
  pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
    let bb = text.as_bytes().to_vec();
    let pat_bb = pattern.as_bytes().to_vec();
    let mut left: Vec<i64> = vec![0; bb.len()];
    let mut right: Vec<i64> = vec![0; bb.len()];
    let mut sum: i64 = 0;
    let mut sums: Vec<i64> = vec![0; bb.len()];
    let mut p1_cnt: i64 = 0;
    let mut p2_cnt: i64 = 0;

    (0..bb.len()).for_each(|idx| {
      if bb[bb.len() - 1 - idx] == pat_bb[1] {
        p2_cnt += 1;
      }
      if bb[idx] == pat_bb[1] {
        sum += p1_cnt;
        sums[idx] = sum;
      }
      if bb[idx] == pat_bb[0] {
        p1_cnt += 1;
      }

      left[idx] = p1_cnt;
      right[bb.len() - 1 - idx] = p2_cnt;
    });
    let mut ans: i64 = 0;
    (0..bb.len()).for_each(|idx| {
      ans = ans.max(sum + left[idx]).max(sum + right[idx]);
    });
    ans
  }
}

fn main() {
  println!(
    "{}",
    Solution::maximum_subsequence_count("aaaa".to_string(), "aa".to_string())
  );
}
